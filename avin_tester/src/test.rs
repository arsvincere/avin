/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::{Path, PathBuf};

use avin_strategy::Strategy;
use bitcode::{Decode, Encode};
use chrono::{DateTime, TimeZone, Utc};

use avin_core::{Iid, TradeList};
use avin_utils::{CFG, Cmd};

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum TestStatus {
    New,
    Edit,
    Process,
    Complete,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Test {
    pub strategy_name: String,
    pub iid: Iid,
    pub deposit: f64,
    pub commission: f64,
    pub begin_ts_nanos: i64,
    pub end_ts_nanos: i64,
    pub status: TestStatus,
    pub trade_list: TradeList,
}
impl Test {
    pub fn new(strategy: &impl Strategy, iid: &Iid) -> Self {
        let trade_list_name = format!("{}_{}", strategy.name(), iid.ticker());

        Self {
            strategy_name: strategy.name().to_string(),
            iid: iid.clone(),
            deposit: 100_000.0,
            commission: 0.0005,
            begin_ts_nanos: Utc
                .with_ymd_and_hms(2020, 1, 1, 0, 0, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap(),
            end_ts_nanos: Utc
                .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap(),
            status: TestStatus::New,
            trade_list: TradeList::new(&trade_list_name),
        }
    }
    pub fn from_bin(bytes: &[u8]) -> Self {
        bitcode::decode(bytes).unwrap()
    }
    pub fn to_bin(&self) -> Vec<u8> {
        bitcode::encode(self)
    }
    pub fn save(test: &Test) -> Result<(), String> {
        let bytes = test.to_bin();
        let path = test.path();
        Cmd::write_bin(&bytes, &path).unwrap();

        log::info!(":: Test save {}", path.display());
        Ok(())
    }
    pub fn load(path: &Path) -> Result<Test, String> {
        let bytes = Cmd::read_bin(path).unwrap();
        let test = Test::from_bin(&bytes);

        log::info!(":: Test load {}", path.display());
        Ok(test)
    }
    pub fn delete(test: &Test) -> Result<(), String> {
        let path = test.path();

        // delete file if exist
        if Cmd::is_exist(&path) {
            Cmd::delete(&path).unwrap();
            log::info!(":: Test delete {}", path.display());
        }

        // delete directory too if empty
        let dir_path = path.parent().unwrap();
        if Cmd::is_empty(dir_path) {
            Cmd::delete_dir(dir_path).unwrap();
        }

        Ok(())
    }

    pub fn name(&self) -> String {
        format!("{}-{}", self.strategy_name, self.iid.ticker())
    }
    pub fn begin(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.begin_ts_nanos)
    }
    pub fn end(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.end_ts_nanos)
    }
    pub fn set_begin(&mut self, dt: &DateTime<Utc>) {
        self.begin_ts_nanos = dt.timestamp_nanos_opt().unwrap();
    }
    pub fn set_end(&mut self, dt: &DateTime<Utc>) {
        self.end_ts_nanos = dt.timestamp_nanos_opt().unwrap();
    }

    pub fn path(&self) -> PathBuf {
        let mut p = PathBuf::new();
        p.push(CFG.dir.test());
        p.push(&self.strategy_name);
        p.push(format!("{}.bin", &self.iid.ticker()));

        p
    }
    pub fn clear(&mut self) {
        self.trade_list.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use avin_core::Asset;
    use avin_strategy::PinBarLong;

    #[test]
    fn new() {
        let strategy = PinBarLong::default();
        let asset = Asset::new("moex_share_ydex").unwrap();
        let test = Test::new(&strategy, asset.iid());

        assert_eq!(test.name(), "PinBarLong-YDEX");
        assert_eq!(test.strategy_name, "PinBarLong");
        assert_eq!(test.iid, *asset.iid());
        assert_eq!(test.deposit, 100_000.0);
        assert_eq!(test.commission, 0.0005);
        assert_eq!(
            test.begin_ts_nanos,
            Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap()
        );
        assert_eq!(
            test.end_ts_nanos,
            Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap()
        );
        assert_eq!(test.status, TestStatus::New);
    }

    #[test]
    fn save_load_delete() {
        let strategy = PinBarLong::default();
        let asset = Asset::new("moex_share_ydex").unwrap();
        let test = Test::new(&strategy, asset.iid());

        // save
        Test::save(&test).unwrap();

        // load
        let path = test.path();
        let loaded = Test::load(&path).unwrap();
        assert_eq!(test, loaded);

        // delete
        Test::delete(&test).unwrap();
        assert!(!Cmd::is_exist(&path));
    }
}
