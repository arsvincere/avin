/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};

use polars::prelude::*;

use super::error::AvinError;

#[derive(Debug)]
pub struct Cmd {}
impl Cmd {
    pub fn is_exist(path: &Path) -> bool {
        path.exists()
    }
    pub fn is_empty(dir_path: &Path) -> bool {
        let files = Cmd::get_files(dir_path).unwrap();

        files.len() == 0
    }

    pub fn name(path: &Path) -> Result<String, AvinError> {
        let file_name = path
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        Ok(file_name)
    }
    pub fn make_dirs(path: &Path) -> Result<(), AvinError> {
        std::fs::create_dir_all(path).unwrap();

        Ok(())
    }
    pub fn get_files(dir_path: &Path) -> Result<Vec<PathBuf>, AvinError> {
        let mut files = Vec::new();
        if dir_path.is_dir() {
            for entry in std::fs::read_dir(dir_path).unwrap() {
                let path = entry.unwrap().path();
                if path.is_file() {
                    files.push(path.to_path_buf());
                }
            }
        }

        files.sort();
        Ok(files)
    }
    pub fn get_dirs(dir_path: &Path) -> Result<Vec<PathBuf>, AvinError> {
        let mut dirs = Vec::new();
        if dir_path.is_dir() {
            for entry in std::fs::read_dir(dir_path).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    dirs.push(path.to_path_buf());
                }
            }
        }

        dirs.sort();
        Ok(dirs)
    }

    pub fn read(path: &Path) -> Result<String, AvinError> {
        let mut file = File::open(path).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();

        Ok(text)
    }
    pub fn read_bin(path: &Path) -> Result<Vec<u8>, AvinError> {
        let mut file = File::open(path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        Ok(bytes)
    }
    pub fn read_pqt(path: &Path) -> Result<DataFrame, AvinError> {
        let mut file = File::open(path).unwrap();
        let df = ParquetReader::new(&mut file).finish().unwrap();

        Ok(df)
    }
    pub fn read_lines(
        path: &Path,
    ) -> io::Result<io::Lines<io::BufReader<File>>> {
        // Returns an Iterator to the Reader of the lines of the file.
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn write(string: &str, path: &Path) -> Result<(), AvinError> {
        let mut file = File::create(path).unwrap();
        write!(file, "{string}").unwrap();

        Ok(())
    }
    pub fn write_bin(bytes: &[u8], path: &Path) -> Result<(), AvinError> {
        // check dir, create if not exist
        let dir_path = path.parent().unwrap();
        if !Cmd::is_exist(dir_path) {
            Cmd::make_dirs(dir_path).unwrap();
        }

        let str_path = path.display();

        // open file in write mode
        let mut file = match File::create(path) {
            Err(why) => panic!("Error create {}: {}", str_path, why),
            Ok(file) => file,
        };

        // write bytes
        if let Err(why) = file.write_all(bytes) {
            panic!("Error save {}: {}", str_path, why);
        }

        Ok(())
    }
    pub fn write_pqt(
        df: &mut DataFrame,
        path: &Path,
    ) -> Result<(), AvinError> {
        let dir_path = path.parent().unwrap();
        if !Cmd::is_exist(dir_path) {
            Cmd::make_dirs(dir_path).unwrap();
        }

        let mut file = File::create(path).unwrap();
        ParquetWriter::new(&mut file).finish(df).unwrap();

        Ok(())
    }

    pub fn delete(path: &Path) -> Result<(), AvinError> {
        std::fs::remove_file(path).unwrap();

        Ok(())
    }
    pub fn delete_dir(path: &Path) -> Result<(), AvinError> {
        std::fs::remove_dir_all(path).unwrap();

        Ok(())
    }
    pub fn replace(from: &Path, to: &Path) -> Result<(), AvinError> {
        std::fs::rename(from, to).unwrap();

        Ok(())
    }
}
