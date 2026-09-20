// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::{fs, fs::File, path::Path};

use polars::prelude::{DataFrame, ParquetReader, ParquetWriter, SerReader};

use crate::StorageError;

/// Returns `true` if the path exists.
///
/// # Errors
///
/// Returns an error if the path metadata cannot be read.
pub fn is_exists(path: &Path) -> Result<bool, StorageError> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(err) => {
            let msg =
                format!("failed to read metadata for '{}'", path.display());
            Err(StorageError::fs(msg, Some(err.into())))
        }
    }
}

/// Returns `true` if the path points to a regular file.
///
/// Returns `false` if the path exists but points to another type of
/// file system entry.
///
/// # Errors
///
/// Returns an error if the path does not exist or if its metadata
/// cannot be read.
pub fn is_file(path: &Path) -> Result<bool, StorageError> {
    let metadata = fs::metadata(path).map_err(|err| {
        let msg = format!("failed to read metadata: {}", path.display());
        StorageError::fs(msg, Some(err.into()))
    })?;

    Ok(metadata.is_file())
}

/// Returns `true` if the path points to a directory.
///
/// Returns `false` if the path exists but points to another type of file system
/// entry.
///
/// # Errors
///
/// Returns an error if the path does not exist or if its metadata cannot be
/// read.
pub fn is_dir(path: &Path) -> Result<bool, StorageError> {
    let metadata = fs::metadata(path).map_err(|err| {
        let msg = format!("failed to read metadata: {}", path.display());
        StorageError::fs(msg, Some(err.into()))
    })?;

    Ok(metadata.is_dir())
}

/// Creates a directory and all missing parent directories.
///
/// # Errors
///
/// Returns an error if the directory or any missing parent directory
/// cannot be created.
pub fn make_dirs(dir_path: &Path) -> Result<(), StorageError> {
    std::fs::create_dir_all(dir_path).map_err(|err| {
        let msg = format!("failed to create dir: {}", dir_path.display());
        StorageError::fs(msg, Some(err.into()))
    })
}

/// Creates all missing parent directories for a file path.
///
/// # Errors
///
/// Returns an error if the file path has no parent directory or if the
/// parent directories cannot be created.
pub fn make_dirs_for_file(file_path: &Path) -> Result<(), StorageError> {
    let dir_path = file_path.parent().ok_or_else(|| {
        let msg = format!("path has no parent dir: {}", file_path.display());
        StorageError::fs(msg, None)
    })?;

    // No parent directories to create if the file is in the current dir.
    if dir_path.as_os_str().is_empty() {
        return Ok(());
    }

    make_dirs(dir_path)
}

/// Deletes a file.
///
/// # Errors
///
/// Returns an error if the file does not exist, cannot be removed, or if
/// the path points to something other than a regular file.
pub fn delete_file(path: &Path) -> Result<(), StorageError> {
    fs::remove_file(path).map_err(|err| {
        let msg = format!("failed to delete file: {}", path.display());
        StorageError::delete(msg, Some(err.into()))
    })
}

/// Deletes a directory and all of its contents recursively.
///
/// # Errors
///
/// Returns an error if the directory does not exist, cannot be removed,
/// or if any file or subdirectory inside it cannot be removed.
pub fn delete_dir(path: &Path) -> Result<(), StorageError> {
    fs::remove_dir_all(path).map_err(|err| {
        let msg = format!("failed to delete directory: {}", path.display());
        StorageError::delete(msg, Some(err.into()))
    })
}

/// Reads a Parquet file into a Polars `DataFrame`.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read.
pub fn read_pqt(path: &Path) -> Result<DataFrame, StorageError> {
    // open file
    let file = File::open(path).map_err(|err| {
        let msg = format!("failed to open file '{}'", path.display());
        StorageError::load(msg, Some(err.into()))
    })?;

    // read DataFrame
    ParquetReader::new(file).finish().map_err(|err| {
        let msg = format!("failed to read file '{}'", path.display());
        StorageError::load(msg, Some(err.into()))
    })
}

/// Writes a Polars `DataFrame` to a Parquet file.
///
/// Missing parent directories are created automatically. Existing file
/// contents are overwritten.
///
/// # Errors
///
/// Returns an error if the parent directories cannot be created, if the
/// file cannot be created, or if Polars cannot write the Parquet data.
// TODO: Реализовать атомарную запись через временный файл и rename,
// чтобы при ошибке записи не потерять существующий кэш.
pub fn write_pqt(
    df: &mut DataFrame,
    path: &Path,
) -> Result<(), StorageError> {
    make_dirs_for_file(path)?;

    // create file
    let file = File::create(path).map_err(|err| {
        let msg = format!("failed to create file '{}'", path.display());
        StorageError::save(msg, Some(err.into()))
    })?;

    // write file
    ParquetWriter::new(file).finish(df).map_err(|err| {
        let msg = format!("failed to write file '{}'", path.display());
        StorageError::save(msg, Some(err.into()))
    })?;

    Ok(())
}
