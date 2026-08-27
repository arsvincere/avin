// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use polars::prelude::{DataFrame, ParquetReader, ParquetWriter, SerReader};

use crate::AvinError;

/// `Cmd` is short for "command" and serves as a convenience namespace for
/// common system and file operations.
///
/// It provides a small, consistent AVIN API for path utilities, file system
/// operations, text and binary I/O, Parquet files, archives, and subprocesses.
///
/// `Cmd` has no state; all operations are exposed as associated functions.
/// Fallible operations return `AvinError` with AVIN-specific context while
/// preserving the underlying source error when available.
pub struct Cmd {}

impl Cmd {
    /// Returns the file stem at the end of the path.
    ///
    /// The stem is the file name without its final extension.
    ///
    /// /home/alex/foo.txt -> "foo"
    /// /home/alex/foo     -> "foo"
    ///
    /// # Errors
    ///
    /// Returns an error if the path has no file stem or if the stem is not
    /// valid UTF-8.
    pub fn stem(path: &Path) -> Result<String, AvinError> {
        let stem = path.file_stem().ok_or_else(|| {
            AvinError::Value(format!(
                "Path has no file stem: {}",
                path.display()
            ))
        })?;

        let stem = stem.to_str().ok_or_else(|| {
            AvinError::Value(format!(
                "Path stem is not valid UTF-8: {}",
                path.display()
            ))
        })?;

        Ok(stem.to_owned())
    }

    /// Returns the file or directory name at the end of the path.
    ///
    /// /home/alex/foo.txt -> "foo.txt"
    /// /home/alex/foo     -> "foo"
    ///
    /// # Errors
    ///
    /// Returns an error if the path has no final component or if its name is not
    /// valid UTF-8.
    pub fn name(path: &Path) -> Result<String, AvinError> {
        let name = path.file_name().ok_or_else(|| {
            AvinError::Value(format!(
                "Path has no file or directory name: {}",
                path.display()
            ))
        })?;

        let name = name.to_str().ok_or_else(|| {
            AvinError::Value(format!(
                "Path name is not valid UTF-8: {}",
                path.display()
            ))
        })?;

        Ok(name.to_owned())
    }

    /// Returns the name of the parent directory in the path.
    ///
    /// The file does not need to exist.
    ///
    /// /home/alex/data/foo.txt → data
    ///
    /// # Errors
    ///
    /// Returns an error if the path has no named parent directory or if the
    /// directory name is not valid UTF-8.
    pub fn dir_name(file_path: &Path) -> Result<String, AvinError> {
        let parent = match file_path.parent() {
            Some(parent) => parent,
            None => {
                return Err(AvinError::Value(format!(
                    "Path has no parent directory: {}",
                    file_path.display()
                )));
            }
        };

        let name = match parent.file_name() {
            Some(name) => name,
            None => {
                return Err(AvinError::Value(format!(
                    "Path has no named parent directory: {}",
                    file_path.display()
                )));
            }
        };

        let name = match name.to_str() {
            Some(name) => name,
            None => {
                return Err(AvinError::Value(format!(
                    "Directory name is not valid UTF-8: {}",
                    parent.display()
                )));
            }
        };

        Ok(name.to_owned())
    }

    /// Returns the parent directory path.
    ///
    /// The file does not need to exist.
    ///
    /// /home/alex/data/foo.txt → /home/alex/data
    ///
    /// # Errors
    ///
    /// Returns an error if the path has no parent directory.
    pub fn dir_path(file_path: &Path) -> Result<PathBuf, AvinError> {
        let parent = match file_path.parent() {
            Some(parent) => parent,
            None => {
                return Err(AvinError::Value(format!(
                    "Path has no parent directory: {}",
                    file_path.display()
                )));
            }
        };

        Ok(parent.to_path_buf())
    }

    /// Returns `true` if the path points to a regular file.
    ///
    /// Returns `false` if the path exists but points to another type of file system
    /// entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the path does not exist or if its metadata cannot be
    /// read.
    pub fn is_file(path: &Path) -> Result<bool, AvinError> {
        let metadata = match std::fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read metadata: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

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
    pub fn is_dir(path: &Path) -> Result<bool, AvinError> {
        let metadata = match std::fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read metadata: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

        Ok(metadata.is_dir())
    }

    /// Returns `true` if the directory contains no entries.
    ///
    /// Any file system entry, including symbolic links, makes the directory
    /// non-empty.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be read or if reading its first
    /// entry fails.
    pub fn is_empty(dir_path: &Path) -> Result<bool, AvinError> {
        let mut iter = match std::fs::read_dir(dir_path) {
            Ok(iter) => iter,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read dir: {}",
                        dir_path.display()
                    ),
                    source: err,
                });
            }
        };

        match iter.next() {
            None => Ok(true),
            Some(Ok(_)) => Ok(false),
            Some(Err(err)) => Err(AvinError::Io {
                message: format!(
                    "Failed to read entry in directory: {}",
                    dir_path.display()
                ),
                source: err,
            }),
        }
    }

    /// Returns files located directly in the given directory.
    ///
    /// Subdirectories are not traversed. Symbolic links to files are not
    /// included.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be read or if an entry's
    /// file type cannot be determined.
    pub fn get_files(dir_path: &Path) -> Result<Vec<PathBuf>, AvinError> {
        let iter = match std::fs::read_dir(dir_path) {
            Ok(iter) => iter,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read dir: {}",
                        dir_path.display()
                    ),
                    source: err,
                });
            }
        };

        let mut files = Vec::new();

        for entry in iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read entry in directory: {}",
                            dir_path.display()
                        ),
                        source: err,
                    });
                }
            };

            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read file type: {}",
                            entry.path().display()
                        ),
                        source: err,
                    });
                }
            };

            if file_type.is_file() {
                files.push(entry.path());
            }
        }

        Ok(files)
    }

    /// Returns directories located directly in the given directory.
    ///
    /// Subdirectories are not traversed recursively. Symbolic links to
    /// directories are not included.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be read or if an entry's file
    /// type cannot be determined.
    pub fn get_dirs(dir_path: &Path) -> Result<Vec<PathBuf>, AvinError> {
        let iter = match std::fs::read_dir(dir_path) {
            Ok(iter) => iter,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read dir: {}",
                        dir_path.display()
                    ),
                    source: err,
                });
            }
        };

        let mut dirs = Vec::new();

        for entry in iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read entry in directory: {}",
                            dir_path.display()
                        ),
                        source: err,
                    });
                }
            };

            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read file type: {}",
                            entry.path().display()
                        ),
                        source: err,
                    });
                }
            };

            if file_type.is_dir() {
                dirs.push(entry.path());
            }
        }

        Ok(dirs)
    }

    /// Returns the contents of a directory as paths.
    ///
    /// The directory is not traversed recursively.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be read or if reading an entry
    /// fails.
    pub fn content(dir_path: &Path) -> Result<Vec<PathBuf>, AvinError> {
        let iter = match std::fs::read_dir(dir_path) {
            Ok(iter) => iter,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read dir: {}",
                        dir_path.display()
                    ),
                    source: err,
                });
            }
        };

        let mut contents = Vec::new();

        for entry in iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read entry in directory: {}",
                            dir_path.display()
                        ),
                        source: err,
                    });
                }
            };

            contents.push(entry.path());
        }

        Ok(contents)
    }

    /// Returns the size of a file in bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the file system metadata cannot be read or if the
    /// path does not point to a regular file.
    pub fn size(path: &Path) -> Result<u64, AvinError> {
        let metadata = match std::fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read metadata: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

        if !metadata.is_file() {
            return Err(AvinError::Value(format!(
                "'{}' is not a file",
                path.display()
            )));
        }

        Ok(metadata.len())
    }

    /// Finds files with the given name recursively in a directory.
    ///
    /// File names must match exactly. Symbolic links are not followed.
    ///
    /// # Errors
    ///
    /// Returns an error if a directory cannot be read, if reading an entry
    /// fails, or if an entry's file type cannot be determined.
    pub fn find_files(
        file_name: &str,
        dir_path: &Path,
    ) -> Result<Vec<PathBuf>, AvinError> {
        let iter = match std::fs::read_dir(dir_path) {
            Ok(iter) => iter,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read dir: {}",
                        dir_path.display()
                    ),
                    source: err,
                });
            }
        };

        let mut found = Vec::new();

        for entry in iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read entry in directory: {}",
                            dir_path.display()
                        ),
                        source: err,
                    });
                }
            };

            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read file type: {}",
                            entry.path().display()
                        ),
                        source: err,
                    });
                }
            };

            let path = entry.path();

            if file_type.is_dir() {
                found.extend(Self::find_files(file_name, &path)?);
            } else if file_type.is_file()
                && entry.file_name() == std::ffi::OsStr::new(file_name)
            {
                found.push(path);
            }
        }

        Ok(found)
    }

    /// Finds directories with the given name recursively in a directory.
    ///
    /// Directory names must match exactly. Symbolic links are not followed.
    ///
    /// # Errors
    ///
    /// Returns an error if a directory cannot be read, if reading an entry
    /// fails, or if an entry's file type cannot be determined.
    pub fn find_dirs(
        dir_name: &str,
        root_dir: &Path,
    ) -> Result<Vec<PathBuf>, AvinError> {
        let iter = match std::fs::read_dir(root_dir) {
            Ok(iter) => iter,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read dir: {}",
                        root_dir.display()
                    ),
                    source: err,
                });
            }
        };

        let mut found = Vec::new();

        for entry in iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read entry in directory: {}",
                            root_dir.display()
                        ),
                        source: err,
                    });
                }
            };

            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read file type: {}",
                            entry.path().display()
                        ),
                        source: err,
                    });
                }
            };

            if file_type.is_dir() {
                let path = entry.path();

                if entry.file_name() == std::ffi::OsStr::new(dir_name) {
                    found.push(path.clone());
                }

                found.extend(Self::find_dirs(dir_name, &path)?);
            }
        }

        Ok(found)
    }

    /// Creates a directory and all missing parent directories.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory or any missing parent directory
    /// cannot be created.
    pub fn make_dirs(dir_path: &Path) -> Result<(), AvinError> {
        match std::fs::create_dir_all(dir_path) {
            Ok(()) => Ok(()),
            Err(err) => Err(AvinError::Io {
                message: format!(
                    "Failed to create directory: {}",
                    dir_path.display()
                ),
                source: err,
            }),
        }
    }

    /// Creates all missing parent directories for a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file path has no parent directory or if the
    /// parent directories cannot be created.
    pub fn make_dirs_for_file(file_path: &Path) -> Result<(), AvinError> {
        let dir_path = match file_path.parent() {
            Some(dir_path) => dir_path,
            None => {
                return Err(AvinError::Value(format!(
                    "File path has no parent directory: {}",
                    file_path.display()
                )));
            }
        };

        if dir_path.as_os_str().is_empty() {
            return Ok(());
        }

        Self::make_dirs(dir_path)
    }

    /// Moves or renames a file system entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the source cannot be moved or renamed to the
    /// destination path.
    pub fn replace(from: &Path, to: &Path) -> Result<(), AvinError> {
        match std::fs::rename(from, to) {
            Ok(()) => Ok(()),
            Err(err) => Err(AvinError::Io {
                message: format!(
                    "Failed to replace '{}' with '{}'",
                    from.display(),
                    to.display()
                ),
                source: err,
            }),
        }
    }

    /// Copies a file to the destination path.
    ///
    /// Creates all missing parent directories for the destination path.
    /// An existing destination file is overwritten.
    ///
    /// # Errors
    ///
    /// Returns an error if the destination directories cannot be created,
    /// if the source file cannot be read, or if the destination file cannot
    /// be created or written.
    pub fn copy_file(
        src_file: &Path,
        dest_file: &Path,
    ) -> Result<(), AvinError> {
        Self::make_dirs_for_file(dest_file)?;

        match std::fs::copy(src_file, dest_file) {
            Ok(_) => Ok(()),
            Err(err) => Err(AvinError::Io {
                message: format!(
                    "Failed to copy '{}' to '{}'",
                    src_file.display(),
                    dest_file.display()
                ),
                source: err,
            }),
        }
    }

    /// Copies a directory and all its contents recursively.
    ///
    /// The destination directory must not already exist. Missing parent
    /// directories are created automatically. Symbolic links are not followed.
    ///
    /// # Errors
    ///
    /// Returns an error if the source directory cannot be read, if the
    /// destination already exists, or if any directory or file cannot be
    /// created or copied.
    pub fn copy_dir(
        src_dir: &Path,
        dest_dir: &Path,
    ) -> Result<(), AvinError> {
        match std::fs::metadata(dest_dir) {
            Ok(_) => {
                return Err(AvinError::Value(format!(
                    "Destination already exists: {}",
                    dest_dir.display()
                )));
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read metadata: {}",
                        dest_dir.display()
                    ),
                    source: err,
                });
            }
        }

        Self::make_dirs(dest_dir)?;

        let iter = match std::fs::read_dir(src_dir) {
            Ok(iter) => iter,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to read dir: {}",
                        src_dir.display()
                    ),
                    source: err,
                });
            }
        };

        for entry in iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read entry in directory: {}",
                            src_dir.display()
                        ),
                        source: err,
                    });
                }
            };

            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read file type: {}",
                            entry.path().display()
                        ),
                        source: err,
                    });
                }
            };

            let src = entry.path();
            let dest = dest_dir.join(entry.file_name());

            if file_type.is_dir() {
                Self::copy_dir(&src, &dest)?;
            } else if file_type.is_file() {
                Self::copy_file(&src, &dest)?;
            }
        }

        Ok(())
    }

    /// Reads a UTF-8 text file into a string.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read, or if its
    /// contents are not valid UTF-8.
    pub fn read(path: &Path) -> Result<String, AvinError> {
        match std::fs::read_to_string(path) {
            Ok(text) => Ok(text),
            Err(err) => Err(AvinError::Io {
                message: format!("Failed to read file: {}", path.display()),
                source: err,
            }),
        }
    }

    /// Opens a text file and returns an iterator over its lines.
    ///
    /// Lines are read lazily and do not include line ending characters.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened. Reading an individual
    /// line may also return an error through the iterator.
    pub fn read_lines(
        path: &Path,
    ) -> Result<impl Iterator<Item = Result<String, AvinError>>, AvinError>
    {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to open file: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

        let path = path.to_path_buf();

        Ok(io::BufReader::new(file).lines().map(move |line| {
            line.map_err(|err| AvinError::Io {
                message: format!(
                    "Failed to read line from file: {}",
                    path.display()
                ),
                source: err,
            })
        }))
    }

    /// Reads a file into a byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read.
    pub fn read_bin(path: &Path) -> Result<Vec<u8>, AvinError> {
        match std::fs::read(path) {
            Ok(bytes) => Ok(bytes),
            Err(err) => Err(AvinError::Io {
                message: format!("Failed to read file: {}", path.display()),
                source: err,
            }),
        }
    }

    /// Reads a Parquet file into a Polars `DataFrame`.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or if Polars cannot read
    /// the Parquet data.
    pub fn read_pqt(path: &Path) -> Result<DataFrame, AvinError> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to open Parquet file: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

        match ParquetReader::new(file).finish() {
            Ok(df) => Ok(df),
            Err(err) => Err(AvinError::Polars {
                message: format!(
                    "Failed to read Parquet file: {}",
                    path.display()
                ),
                source: err,
            }),
        }
    }

    /// Writes a string to a file, overwriting its existing contents.
    ///
    /// Missing parent directories are created automatically.
    ///
    /// # Errors
    ///
    /// Returns an error if the parent directories cannot be created or if the
    /// file cannot be created or written.
    pub fn write(string: &str, path: &Path) -> Result<(), AvinError> {
        Self::make_dirs_for_file(path)?;

        match std::fs::write(path, string) {
            Ok(()) => Ok(()),
            Err(err) => Err(AvinError::Io {
                message: format!("Failed to write file: {}", path.display()),
                source: err,
            }),
        }
    }

    /// Writes lines to a text file, overwriting its existing contents.
    ///
    /// Strings are written as-is. Line ending characters are not added
    /// automatically. Missing parent directories are created automatically.
    ///
    /// # Errors
    ///
    /// Returns an error if the parent directories cannot be created or if the
    /// file cannot be created or written.
    pub fn write_lines(
        lines: &[String],
        path: &Path,
    ) -> Result<(), AvinError> {
        Self::make_dirs_for_file(path)?;

        let file = match File::create(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to create file: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

        let mut writer = io::BufWriter::new(file);

        for line in lines {
            if let Err(err) = writer.write_all(line.as_bytes()) {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to write file: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        }

        Ok(())
    }

    /// Writes bytes to a file, overwriting its existing contents.
    ///
    /// Missing parent directories are created automatically.
    ///
    /// # Errors
    ///
    /// Returns an error if the parent directories cannot be created or if the
    /// file cannot be created or written.
    pub fn write_bin(bytes: &[u8], path: &Path) -> Result<(), AvinError> {
        Self::make_dirs_for_file(path)?;

        match std::fs::write(path, bytes) {
            Ok(()) => Ok(()),
            Err(err) => Err(AvinError::Io {
                message: format!(
                    "Failed to write binary file: {}",
                    path.display()
                ),
                source: err,
            }),
        }
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
    pub fn write_pqt(
        df: &mut DataFrame,
        path: &Path,
    ) -> Result<(), AvinError> {
        Self::make_dirs_for_file(path)?;

        let file = match File::create(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to create Parquet file: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

        match ParquetWriter::new(file).finish(df) {
            Ok(_) => Ok(()),
            Err(err) => Err(AvinError::Polars {
                message: format!(
                    "Failed to write Parquet file: {}",
                    path.display()
                ),
                source: err,
            }),
        }
    }

    /// Appends a string to an existing text file.
    ///
    /// The string is written as-is. Line ending characters are not added
    /// automatically.
    ///
    /// # Errors
    ///
    /// Returns an error if the file does not exist, cannot be opened, or cannot be
    /// written.
    pub fn append(text: &str, path: &Path) -> Result<(), AvinError> {
        let mut file =
            match std::fs::OpenOptions::new().append(true).open(path) {
                Ok(file) => file,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to open file for append: {}",
                            path.display()
                        ),
                        source: err,
                    });
                }
            };

        match file.write_all(text.as_bytes()) {
            Ok(()) => Ok(()),
            Err(err) => Err(AvinError::Io {
                message: format!(
                    "Failed to append to file: {}",
                    path.display()
                ),
                source: err,
            }),
        }
    }

    /// Appends strings to an existing text file.
    ///
    /// Strings are written as-is. Line ending characters are not added
    /// automatically.
    ///
    /// # Errors
    ///
    /// Returns an error if the file does not exist, cannot be opened, or
    /// cannot be written.
    pub fn append_lines(
        lines: &[String],
        path: &Path,
    ) -> Result<(), AvinError> {
        let file = match std::fs::OpenOptions::new().append(true).open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to open file for append: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

        let mut writer = io::BufWriter::new(file);

        for line in lines {
            if let Err(err) = writer.write_all(line.as_bytes()) {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to append to file: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        }

        Ok(())
    }

    /// Returns the last `n` lines of a text file.
    ///
    /// The file is read sequentially and only the last `n` lines are kept in
    /// memory. Line ending characters are preserved.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read.
    pub fn get_tail(path: &Path, n: usize) -> Result<Vec<String>, AvinError> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to open file: {}",
                        path.display()
                    ),
                    source: err,
                });
            }
        };

        if n == 0 {
            return Ok(Vec::new());
        }

        let mut reader = io::BufReader::new(file);
        let mut tail = std::collections::VecDeque::with_capacity(n);

        loop {
            let mut line = String::new();

            let bytes = match reader.read_line(&mut line) {
                Ok(bytes) => bytes,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to read file: {}",
                            path.display()
                        ),
                        source: err,
                    });
                }
            };

            if bytes == 0 {
                break;
            }

            if tail.len() == n {
                tail.pop_front();
            }

            tail.push_back(line);
        }

        Ok(tail.into_iter().collect())
    }

    /// Deletes a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file does not exist, cannot be removed, or if
    /// the path points to something other than a regular file.
    pub fn delete(path: &Path) -> Result<(), AvinError> {
        match std::fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(err) => Err(AvinError::Io {
                message: format!("Failed to delete file: {}", path.display()),
                source: err,
            }),
        }
    }

    /// Deletes a directory and all of its contents recursively.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory does not exist, cannot be removed,
    /// or if any file or subdirectory inside it cannot be removed.
    pub fn delete_dir(path: &Path) -> Result<(), AvinError> {
        match std::fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(err) => Err(AvinError::Io {
                message: format!(
                    "Failed to delete directory: {}",
                    path.display()
                ),
                source: err,
            }),
        }
    }

    /// Extracts a ZIP archive into a directory.
    ///
    /// Existing files in the destination directory may be overwritten.
    ///
    /// # Errors
    ///
    /// Returns an error if the archive cannot be opened, read, or extracted.
    pub fn extract_zip(
        archive_path: &Path,
        dest_dir: &Path,
    ) -> Result<(), AvinError> {
        let file = match File::open(archive_path) {
            Ok(file) => file,
            Err(err) => {
                return Err(AvinError::Io {
                    message: format!(
                        "Failed to open ZIP archive: {}",
                        archive_path.display()
                    ),
                    source: err,
                });
            }
        };

        let mut archive = match zip::ZipArchive::new(file) {
            Ok(archive) => archive,
            Err(err) => {
                return Err(AvinError::Zip {
                    message: format!(
                        "Failed to read ZIP archive: {}",
                        archive_path.display()
                    ),
                    source: err,
                });
            }
        };

        match archive.extract(dest_dir) {
            Ok(()) => Ok(()),
            Err(err) => Err(AvinError::Zip {
                message: format!(
                    "Failed to extract ZIP archive '{}' to '{}'",
                    archive_path.display(),
                    dest_dir.display()
                ),
                source: err,
            }),
        }
    }

    /// Runs a subprocess and waits for it to finish.
    ///
    /// The first item is treated as the program name and the remaining items
    /// as its arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is empty, if the process cannot be
    /// started, or if it exits with a non-zero status.
    pub fn subprocess(command: &[String]) -> Result<(), AvinError> {
        let (program, args) = match command.split_first() {
            Some(parts) => parts,
            None => {
                return Err(AvinError::Value(
                    "Subprocess command cannot be empty".to_owned(),
                ));
            }
        };

        let status =
            match std::process::Command::new(program).args(args).status() {
                Ok(status) => status,
                Err(err) => {
                    return Err(AvinError::Io {
                        message: format!(
                            "Failed to start subprocess: {}",
                            command.join(" ")
                        ),
                        source: err,
                    });
                }
            };

        if !status.success() {
            return Err(AvinError::Process(format!(
                "Subprocess failed with status {status}: {}",
                command.join(" ")
            )));
        }

        Ok(())
    }
}
