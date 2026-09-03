// /// Reads a Parquet file into a Polars `DataFrame`.
// ///
// /// # Errors
// ///
// /// Returns an error if the file cannot be opened or if Polars cannot read
// /// the Parquet data.
// pub fn read_pqt(path: &Path) -> Result<DataFrame, AvinError> {
//     let file = match File::open(path) {
//         Ok(file) => file,
//         Err(err) => {
//             return Err(AvinError::Io {
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
//         Err(err) => Err(AvinError::Polars {
//             message: format!(
//                 "Failed to read Parquet file: {}",
//                 path.display()
//             ),
//             source: err,
//         }),
//     }
// }
//

// /// Writes a Polars `DataFrame` to a Parquet file.
// ///
// /// Missing parent directories are created automatically. Existing file
// /// contents are overwritten.
// ///
// /// # Errors
// ///
// /// Returns an error if the parent directories cannot be created, if the
// /// file cannot be created, or if Polars cannot write the Parquet data.
// pub fn write_pqt(
//     df: &mut DataFrame,
//     path: &Path,
// ) -> Result<(), AvinError> {
//     Self::make_dirs_for_file(path)?;
//
//     let file = match File::create(path) {
//         Ok(file) => file,
//         Err(err) => {
//             return Err(AvinError::Io {
//                 message: format!(
//                     "Failed to create Parquet file: {}",
//                     path.display()
//                 ),
//                 source: err,
//             });
//         }
//     };
//
//     match ParquetWriter::new(file).finish(df) {
//         Ok(_) => Ok(()),
//         Err(err) => Err(AvinError::Polars {
//             message: format!(
//                 "Failed to write Parquet file: {}",
//                 path.display()
//             ),
//             source: err,
//         }),
//     }
// }
//
