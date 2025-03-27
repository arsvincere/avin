use crate::data::data_file_bar::DataFileBar;
use crate::data::instrument::Instrument;
use crate::data::market_data::MarketData;
use crate::data::source::Source;
use crate::data::source_moex::SourceMoex;
use chrono::prelude::*;

pub struct Manager {}
impl Manager {
    pub async fn download(
        source: &Source,
        instrument: &Instrument,
        market_data: &MarketData,
    ) -> Result<(), &'static str> {
        let source = match source {
            Source::MOEX => SourceMoex::new(),
            Source::TINKOFF => panic!("Нахер с Тинькофф качать?"),
            Source::CONVERTER => panic!(),
        };
        println!(":: Download {} {}", instrument.ticker, market_data.name());

        let mut year: i32 = 1990; // суть - более старых данных точно нет
        let now_year = Utc::now().year();

        while year <= now_year {
            let begin = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
            let end = Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap();
            let df = source
                .get_bars(&instrument, &market_data, &begin, &end)
                .await?;

            if df.is_empty() {
                println!("   - no data for {year}");
                year += 1;
                continue;
            }

            // INFO: ParquetWriter требует &mut df для сохранения...
            // по факту никто data_file не меняет перед записью
            let mut data_file = DataFileBar::new(
                instrument.clone(),
                market_data.clone(),
                df,
                year,
            )
            .unwrap();
            DataFileBar::save(&mut data_file)?;
            year += 1;
        }

        println!("Download complete!");
        Ok(())
    }
}
