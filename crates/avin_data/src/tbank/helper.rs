// /// Extracts a ZIP archive into a directory.
// ///
// /// Existing files in the destination directory may be overwritten.
// ///
// /// # Errors
// ///
// /// Returns an error if the archive cannot be opened, read, or extracted.
// pub fn extract_zip(
//     archive_path: &Path,
//     dest_dir: &Path,
// ) -> Result<(), AvinError> {
//     let file = match File::open(archive_path) {
//         Ok(file) => file,
//         Err(err) => {
//             return Err(AvinError::Io {
//                 message: format!(
//                     "Failed to open ZIP archive: {}",
//                     archive_path.display()
//                 ),
//                 source: err,
//             });
//         }
//     };
//
//     let mut archive = match zip::ZipArchive::new(file) {
//         Ok(archive) => archive,
//         Err(err) => {
//             return Err(AvinError::Zip {
//                 message: format!(
//                     "Failed to read ZIP archive: {}",
//                     archive_path.display()
//                 ),
//                 source: err,
//             });
//         }
//     };
//
//     match archive.extract(dest_dir) {
//         Ok(()) => Ok(()),
//         Err(err) => Err(AvinError::Zip {
//             message: format!(
//                 "Failed to extract ZIP archive '{}' to '{}'",
//                 archive_path.display(),
//                 dest_dir.display()
//             ),
//             source: err,
//         }),
//     }
// }
//
