/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
// use std::str::FromStr;

#[derive(Debug)]
pub struct Cmd {}
impl Cmd {
    // pub fn is_exist(path: &Path) -> bool {
    //     todo!("is exist file path");
    // }
    // pub fn path(parts: &Vec<&Path>) {
    //     todo!("join path from Vec")
    // }
    pub fn make_dirs(path: &Path) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(path)?;

        Ok(())
    }

    pub fn get_files(
        dir_path: &PathBuf,
    ) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut files = Vec::new();
        if dir_path.is_dir() {
            for entry in std::fs::read_dir(dir_path)? {
                let path = entry?.path();
                if path.is_file() {
                    files.push(path.to_path_buf());
                }
            }
        }

        files.sort();
        Ok(files)
    }

    pub fn read(path: &Path) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(&path)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        Ok(text)
    }
    pub fn read_pqt(path: &Path) -> Result<DataFrame, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let df = ParquetReader::new(&mut file).finish()?;

        Ok(df)
    }
    pub fn read_lines(
        path: &Path,
    ) -> io::Result<io::Lines<io::BufReader<File>>> {
        // Returns an Iterator to the Reader of the lines of the file.
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn write(string: &str, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        write!(file, "{string}")?;

        Ok(())
    }
    pub fn write_pqt(
        df: &mut DataFrame,
        path: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let dir_path = path.parent().unwrap();
        std::fs::create_dir_all(dir_path)?;

        let mut file = File::create(path)?;
        ParquetWriter::new(&mut file).finish(df).unwrap();

        Ok(())
    }
    // pub fn write_lines(lines: Vector<&str>) -> Result<(), Box<dyn Error>> {
    //     // ...
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_read_write() {
//         let string = String::from("Hello!");
//         let path = Path::new("./tmp/cmd_read_write.txt");
//
//         let _ = Cmd::write(&string, &path).expect("WTF?");
//
//         let readed = Cmd::read(&path).expect("WTF?");
//         assert_eq!(string, readed);
//     }
// }
