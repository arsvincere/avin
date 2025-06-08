/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::data::IID;
use crate::data::error::DataError;
use crate::data::market_data::MarketData;
use crate::utils::Cmd;
use polars::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DataFileBar {
    pub iid: IID,
    pub market_data: MarketData,
    pub data: DataFrame,
    pub year: i32,
}
impl DataFileBar {
    pub fn path(&self) -> PathBuf {
        let mut path = self.iid.path();
        path.push(&self.market_data.name());
        path.push(format!("{}.pqt", self.year.to_string()));

        // return format!("{asset_path}/{market_data}/{year}.pqt");
        path
    }

    pub fn new(
        iid: &IID,
        market_data: MarketData,
        data: DataFrame,
        year: i32,
    ) -> Result<DataFileBar, &'static str> {
        // TODO: проверка что begin end в пределах файла в одном году
        // находятся
        // let begin = data.column("dt").unwrap().get(0).unwrap();
        // let end = data.column("dt").unwrap().len();
        // let end = data.column("dt").unwrap().get(end - 1).unwrap();

        let data_file = DataFileBar {
            iid: iid.clone(),
            market_data,
            data,
            year,
        };
        Ok(data_file)
    }
    pub fn save(data_file: &mut DataFileBar) -> Result<(), &'static str> {
        let file_path = data_file.path();
        Cmd::write_pqt(&mut data_file.data, &file_path).unwrap();

        println!("   save {}", file_path.display());
        Ok(())
    }
    pub fn load(
        iid: &IID,
        market_data: &MarketData,
        year: i32,
    ) -> Result<DataFrame, DataError> {
        // get path
        let mut path = iid.path();
        path.push(&market_data.name());
        path.push(format!("{year}.pqt"));

        if !Cmd::is_exist(&path) {
            let msg = format!("{} {}", iid, market_data);
            return Err(DataError::NotFound(msg.to_string()));
        }

        match Cmd::read_pqt(&path) {
            Ok(df) => Ok(df),
            Err(why) => {
                let msg = format!("read {} - {}", path.display(), why);
                Err(DataError::ReadError(msg.to_string()))
            }
        }

        // let data_file = DataFileBar::new(
        //     iid.clone(),
        //     market_data.clone(),
        //     df,
        //     year,
        // )
        // .unwrap();

        // Ok(data_file)
    }
    pub fn request_all(
        iid: &IID,
        market_data: &MarketData,
    ) -> Result<Vec<DataFileBar>, &'static str> {
        // dir path
        let mut dir_path = iid.path();
        dir_path.push(&market_data.name());

        // get files
        let file_paths = Cmd::get_files(&dir_path).unwrap();

        // read parquet files & create DataFileBar objs
        let mut all_data_files = Vec::new();
        for path in file_paths {
            let year: i32 = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            let df = Cmd::read_pqt(&path).unwrap();
            let data_file =
                DataFileBar::new(iid, market_data.clone(), df, year).unwrap();

            all_data_files.push(data_file);
        }

        Ok(all_data_files)
    }
}
