/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use avin_core::{Chart, TimeFrame};
use avin_utils::{AvinError, CFG, Cmd};

use crate::{Condition, Marker, Point};

#[derive(Debug, Deserialize, Serialize)]
pub struct FilterResult {
    scan_name: String,
    iid_name: String,
    tf: TimeFrame,
    marker: Marker,
    points: Vec<Point>,
}
impl FilterResult {
    pub fn new(
        chart: &Chart,
        filter: impl Condition,
        marker: Marker,
        points: Vec<Point>,
    ) -> Self {
        Self {
            scan_name: format!("{}_{}", filter.name(), chart.ticker()),
            iid_name: chart.iid().to_string(),
            tf: chart.tf(),
            marker,
            points,
        }
    }
    pub fn save(result: &FilterResult) -> Result<(), AvinError> {
        let text = toml::to_string_pretty(result).unwrap();
        let mut path = CFG.dir.scan();
        path.push(format!("{}.toml", result.scan_name));

        Cmd::write(&text, &path).unwrap();

        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self, AvinError> {
        let text = Cmd::read(path).unwrap();
        let r: FilterResult = toml::from_str(&text).unwrap();

        Ok(r)
    }

    pub fn scan_name(&self) -> &String {
        &self.scan_name
    }
    pub fn iid_name(&self) -> &String {
        &self.iid_name
    }
    pub fn tf(&self) -> TimeFrame {
        self.tf
    }
    pub fn marker(&self) -> &Marker {
        &self.marker
    }
    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
    pub fn begin(&self) -> DateTime<Utc> {
        let ts = self.points.first().unwrap().ts;
        DateTime::from_timestamp_nanos(
            ts - 24 * 60 * 60 * 1_000_000_000, // -1 day
        )
    }
    pub fn end(&self) -> DateTime<Utc> {
        let ts = self.points.last().unwrap().ts;
        DateTime::from_timestamp_nanos(
            ts + 24 * 60 * 60 * 1_000_000_000, // +1 day
        )
    }
}

pub struct FilterResultList {
    scanner_results: Vec<FilterResult>,
}
impl Default for FilterResultList {
    fn default() -> Self {
        FilterResultList::new()
    }
}
impl FilterResultList {
    pub fn new() -> Self {
        Self {
            scanner_results: Vec::new(),
        }
    }
    pub fn save(list: &FilterResultList) -> Result<(), AvinError> {
        for result in list.scanner_results.iter() {
            FilterResult::save(result).unwrap();
        }

        Ok(())
    }
    pub fn load_name(name: &str) -> Result<FilterResultList, AvinError> {
        // create empty scanner result list
        let mut scan_list = FilterResultList::new();

        // create dir path
        let mut dir_path = CFG.dir.scan();
        dir_path.push(name);
        let files = Cmd::get_files(&dir_path).unwrap();

        // load scan files
        for file in files {
            let scan = FilterResult::load(&file).unwrap();
            scan_list.add(scan);
        }

        Ok(scan_list)
    }
    pub fn load_dir(path: &Path) -> Result<FilterResultList, AvinError> {
        // create empty scan list
        let mut test_list = FilterResultList::new();

        // get scan paths of scan files
        let files = Cmd::get_files(path).unwrap();

        // load scan files
        for file in files {
            let scan = FilterResult::load(&file).unwrap();
            test_list.add(scan);
        }

        Ok(test_list)
    }

    pub fn is_empty(&self) -> bool {
        self.scanner_results.is_empty()
    }
    pub fn len(&self) -> usize {
        self.scanner_results.len()
    }
    pub fn results(&self) -> &Vec<FilterResult> {
        &self.scanner_results
    }
    pub fn add(&mut self, result: FilterResult) {
        self.scanner_results.push(result);
    }
    pub fn get(&self, index: usize) -> Option<&FilterResult> {
        self.scanner_results.get(index)
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut FilterResult> {
        self.scanner_results.get_mut(index)
    }
    pub fn clear(&mut self) {
        self.scanner_results.clear();
    }
}
