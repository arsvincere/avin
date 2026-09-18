// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::{fs::File, path::Path};

use polars::prelude::{DataFrame, ParquetWriter};

use crate::StorageError;

// /// Reads a Parquet file into a Polars `DataFrame`.
// ///
// /// # Errors
// ///
// /// Returns an error if the file cannot be opened or if Polars cannot read
// /// the Parquet data.
// pub fn read_pqt(path: &Path) -> Result<DataFrame, StorageError> {
//     let file = match File::open(path) {
//         Ok(file) => file,
//         Err(err) => {
//             return Err(StorageError::Io {
//                 message: format!(
//                     "Failed to open Parquet file: {}",
//                     path.display()
//                 ),
//                 source: err,
//             });
//         }
//     };
//
//     match ParquetReader::new(file).finish() {
//         Ok(df) => Ok(df),
//         Err(err) => Err(StorageError::Polars {
//             message: format!(
//                 "Failed to read Parquet file: {}",
//                 path.display()
//             ),
//             source: err,
//         }),
//     }
// }

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

    let file = File::create(path).map_err(|err| {
        let msg = format!("failed to create file '{}'", path.display());
        StorageError::save(msg, Some(err.into()))
    })?;

    ParquetWriter::new(file).finish(df).map_err(|err| {
        let msg = format!("failed to write file '{}'", path.display());
        StorageError::save(msg, Some(err.into()))
    })?;

    Ok(())
}

pub fn make_dirs(dir_path: &Path) -> Result<(), StorageError> {
    std::fs::create_dir_all(dir_path).map_err(|err| {
        let msg = format!("failed to create dir: {}", dir_path.display());
        StorageError::save(msg, Some(err.into()))
    })
}

pub fn make_dirs_for_file(file_path: &Path) -> Result<(), StorageError> {
    let dir_path = file_path.parent().ok_or_else(|| {
        let msg = format!("path has no parent dir: {}", file_path.display());
        StorageError::save(msg, None)
    })?;

    // No parent directories to create if the file is in the current dir.
    if dir_path.as_os_str().is_empty() {
        return Ok(());
    }

    make_dirs(dir_path)
}
