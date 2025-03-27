use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;

#[derive(Debug)]
pub struct Cmd {}
impl Cmd {
    pub fn is_exist(path: &str) -> bool {
        dbg!(&path);
        todo!("is exist file path");
    }
    pub fn path(parts: &Vec<&str>) {
        dbg!(&parts);
        todo!("join path from Vec")
    }
    pub fn make_dirs(path: &str) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(path)?;

        Ok(())
    }
    pub fn read(path: &str) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(&path)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        Ok(text)
    }
    pub fn write(string: &str, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        write!(file, "{string}")?;

        Ok(())
    }
    pub fn write_pqt(
        df: &mut DataFrame,
        path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let path = Path::new(path);
        let dir_path = path.parent().unwrap();
        std::fs::create_dir_all(dir_path)?;

        let mut file = File::create(path)?;
        ParquetWriter::new(&mut file).finish(df).unwrap();

        Ok(())
    }
    pub fn read_lines(
        path: &str,
    ) -> io::Result<io::Lines<io::BufReader<File>>> {
        // Returns an Iterator to the Reader of the lines of the file.
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }
    // pub fn write_lines(lines: Vector<&str>) -> Result<(), Box<dyn Error>> {
    //     // ...
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let string = String::from("Hello!");
        let path = "./tmp/cmd_read_write.txt";
        let _ = Cmd::write(&string, &path).expect("WTF?");

        let readed = Cmd::read(&path).expect("WTF?");
        assert_eq!(string, readed);
    }
}
