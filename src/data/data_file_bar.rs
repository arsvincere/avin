use crate::data::instrument::Instrument;
use crate::data::market_data::MarketData;
use crate::utils::Cmd;
use polars::prelude::*;

#[derive(Debug)]
pub struct DataFileBar {
    instrument: Instrument,
    market_data: MarketData,
    data: DataFrame,
    year: i32,
}
impl DataFileBar {
    pub fn new(
        instrument: Instrument,
        market_data: MarketData,
        data: DataFrame,
        year: i32,
    ) -> Result<DataFileBar, &'static str> {
        // TODO: проверка что begin end в пределах файла в одном году
        // находятся
        // let begin = data.column("dt").unwrap().get(0).unwrap();
        // let end = data.column("dt").unwrap().len();
        // let end = data.column("dt").unwrap().get(end - 1).unwrap();
        // dbg!(begin);
        // dbg!(end);

        let data_file = DataFileBar {
            instrument,
            market_data,
            data,
            year,
        };
        Ok(data_file)
    }
    pub fn save(data_file: &mut DataFileBar) -> Result<(), &'static str> {
        let file_path = data_file.path();
        Cmd::write_pqt(&mut data_file.data, &file_path).unwrap();

        println!("   save {}", file_path);
        Ok(())
    }
    pub fn path(&self) -> String {
        let instrument_path = self.instrument.path();
        let market_data = self.market_data.name();
        let year = self.year.to_string();

        return format!("{instrument_path}/{market_data}/{year}.pqt");
    }
}
