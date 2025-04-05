/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::conf::DATA_DIR;
use crate::conf::DEFAULT_BARS_COUNT;
use crate::core::chart::Chart;
use crate::core::timeframe::TimeFrame;
use bitcode::{Decode, Encode};
use chrono::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Asset {
    pub exchange: String,
    pub itype: String,
    pub ticker: String,

    charts: HashMap<TimeFrame, Chart>,
}
impl Asset {
    pub fn from(s: &str) -> Result<Asset, &'static str> {
        let parts: Vec<&str> = s.split("_").collect();
        if parts.len() != 3 {
            eprintln!("Fail to create asset from str: {s}");
            return Err("Invalid asset");
        };

        // TODO: пока работает только биржа MOEX
        let exchange = parts[0].to_uppercase();
        assert_eq!(exchange, "MOEX");

        // TODO: пока работает только тип инструмента SHARE
        let itype = parts[1].to_uppercase();
        assert_eq!(itype, "SHARE");

        // TODO: пока не сделал кэширование информации о доступных
        // инструментах работает только ниже указанные тикеры
        let ticker = parts[2].to_uppercase();
        assert!("GAZP LKOH MOEX ROSN SBER VTBR YDEX".contains(&ticker));

        let charts = HashMap::new();

        let asset = Asset {
            exchange,
            itype,
            ticker,
            charts,
        };
        Ok(asset)
    }
    pub fn to_string(&self) -> String {
        format!("{}_{}_{}", self.exchange, self.itype, self.ticker)
    }

    pub fn chart(&self, tf: &TimeFrame) -> Option<&Chart> {
        self.charts.get(tf)
    }
    pub fn load_chart(
        &mut self,
        tf: &TimeFrame,
    ) -> Result<&Chart, &'static str> {
        let end = Utc::now();
        let begin = end - tf.timedelta() * DEFAULT_BARS_COUNT;

        return self.load_chart_period(tf, &begin, &end);
    }
    pub fn load_chart_period(
        &mut self,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, &'static str> {
        let chart = Chart::load(&self, tf, begin, end)?;
        self.charts.insert(tf.clone(), chart);

        Ok(self.charts[tf].as_ref())
    }
    pub fn path(&self) -> PathBuf {
        let mut p = std::path::PathBuf::new();
        p.push(&DATA_DIR);
        p.push(&self.exchange);
        p.push(&self.itype);
        p.push(&self.ticker);

        return p;
    }
    pub fn copy_id(&self) -> Asset {
        Asset {
            exchange: self.exchange.clone(),
            itype: self.itype.clone(),
            ticker: self.ticker.clone(),
            charts: HashMap::new(),
        }
    }
}
impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Asset={} {} {}", self.exchange, self.itype, self.ticker)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_chart() {
        let mut asset = Asset::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("1H");
        let begin = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();

        let chart = asset.load_chart_period(&tf, &begin, &end).unwrap();

        assert_eq!(chart.tf(), &tf);
        assert_eq!(
            chart.first().unwrap().dt(),
            Utc.with_ymd_and_hms(2025, 1, 3, 6, 0, 0).unwrap()
        );
        assert_eq!(
            chart.last().unwrap().dt(),
            Utc.with_ymd_and_hms(2025, 1, 31, 20, 0, 0).unwrap()
        );
    }
    #[test]
    fn load_chart_no_args() {
        let mut asset = Asset::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");

        let chart = asset.load_chart(&tf).unwrap();
        assert_eq!(chart.tf(), &tf);

        assert!(chart.bars().len() > 1000);
        assert!(chart.bars().len() < 5000);
    }
}
