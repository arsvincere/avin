/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

// TODO: Filter можно переименовать в Condition, и модуль в целом
// переименовать в avin_filter. Тогда освободится имя avin_scanner
// и его займет advisor. Так более логичный нейминг получится.

use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use avin_analyse::TrendAnalytic;
use avin_core::{Chart, ExtremumIndicator, TimeFrame};
use avin_utils::{AvinError, CFG, Cmd};

pub trait Filter {
    fn name(&self) -> &'static str;
    fn apply(&self, chart: &Chart) -> bool;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarkerShape {
    Circle,
    Square,
    Diamond,
    Plus,
    Cross,
    Asterisk,
    Up,
    Down,
    Left,
    Right,
}
impl std::fmt::Display for MarkerShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Circle => write!(f, "Circle"),
            Self::Square => write!(f, "Square"),
            Self::Diamond => write!(f, "Diamond"),
            Self::Plus => write!(f, "Plus"),
            Self::Cross => write!(f, "Cross"),
            Self::Asterisk => write!(f, "Asterisk"),
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarkerColor {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Violet,
    White,
    Grey,
    Black,
}
impl std::fmt::Display for MarkerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "#FF0000"),
            Self::Orange => write!(f, "#FFA500"),
            Self::Yellow => write!(f, "#FFFF00"),
            Self::Green => write!(f, "#008000"),
            Self::Cyan => write!(f, "#00FFFF"),
            Self::Blue => write!(f, "#0000FF"),
            Self::Violet => write!(f, "#EE82EE"),
            Self::White => write!(f, "#FFFFFF"),
            Self::Grey => write!(f, "#808080"),
            Self::Black => write!(f, "#000000"),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarkerSize {
    Small = 3,
    Medium = 5,
    Large = 8,
}
impl std::fmt::Display for MarkerSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small => write!(f, "S"),
            Self::Medium => write!(f, "M"),
            Self::Large => write!(f, "L"),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Marker {
    pub shape: MarkerShape,
    pub color: MarkerColor,
    pub size: MarkerSize,
}
impl Marker {
    pub fn new(
        shape: MarkerShape,
        color: MarkerColor,
        size: MarkerSize,
    ) -> Self {
        Self { shape, color, size }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub ts: i64,
    pub price: f64,
}
impl Point {
    pub fn new(ts: i64, price: f64) -> Self {
        Self { ts, price }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScannerResult {
    scan_name: String,
    iid_name: String,
    tf: TimeFrame,
    marker: Marker,
    points: Vec<Point>,
}
impl ScannerResult {
    pub fn new(
        chart: &Chart,
        filter: impl Filter,
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
    pub fn save(result: &ScannerResult) -> Result<(), AvinError> {
        let text = toml::to_string_pretty(result).unwrap();
        let mut path = CFG.dir.scan();
        path.push(format!("{}.toml", result.scan_name));

        Cmd::write(&text, &path).unwrap();

        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self, AvinError> {
        let text = Cmd::read(path).unwrap();
        let r: ScannerResult = toml::from_str(&text).unwrap();

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

pub struct ScannerResultList {
    scanner_results: Vec<ScannerResult>,
}
impl Default for ScannerResultList {
    fn default() -> Self {
        ScannerResultList::new()
    }
}
impl ScannerResultList {
    pub fn new() -> Self {
        Self {
            scanner_results: Vec::new(),
        }
    }
    pub fn save(list: &ScannerResultList) -> Result<(), AvinError> {
        for result in list.scanner_results.iter() {
            ScannerResult::save(result).unwrap();
        }

        Ok(())
    }
    pub fn load_name(name: &str) -> Result<ScannerResultList, AvinError> {
        // create empty scanner result list
        let mut scan_list = ScannerResultList::new();

        // create dir path
        let mut dir_path = CFG.dir.scan();
        dir_path.push(name);
        let files = Cmd::get_files(&dir_path).unwrap();

        // load scan files
        for file in files {
            let scan = ScannerResult::load(&file).unwrap();
            scan_list.add(scan);
        }

        Ok(scan_list)
    }
    pub fn load_dir(path: &Path) -> Result<ScannerResultList, AvinError> {
        // create empty scan list
        let mut test_list = ScannerResultList::new();

        // get scan paths of scan files
        let files = Cmd::get_files(path).unwrap();

        // load scan files
        for file in files {
            let scan = ScannerResult::load(&file).unwrap();
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
    pub fn results(&self) -> &Vec<ScannerResult> {
        &self.scanner_results
    }
    pub fn add(&mut self, result: ScannerResult) {
        self.scanner_results.push(result);
    }
    pub fn get(&self, index: usize) -> Option<&ScannerResult> {
        self.scanner_results.get(index)
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ScannerResult> {
        self.scanner_results.get_mut(index)
    }
    pub fn clear(&mut self) {
        self.scanner_results.clear();
    }
}

pub struct Scanner {}
impl Scanner {
    pub fn scan(
        chart: &Chart,
        filter: impl Filter,
        marker: Marker,
    ) -> Result<(), AvinError> {
        // временный вектор для найденных точек, где фильтр сработал
        let mut points = Vec::new();

        // первое нужен пустой график того же актива и таймфрейма
        let mut new_chart = Chart::empty(chart.iid(), chart.tf());
        ExtremumIndicator::init(&mut new_chart);
        TrendAnalytic::init(&mut new_chart);

        // берем бары от переданного графика
        let bars = chart.bars();

        // добавляем эти бары поштучно в пустой график и чекаем фильтр
        for bar in bars.iter() {
            new_chart.add_bar(*bar);

            let result = filter.apply(&new_chart);

            if result {
                let ts = new_chart.now().unwrap().ts;
                let price = new_chart.now().unwrap().h * 1.003;
                points.push(Point::new(ts, price));
            }
        }

        // сохраняем результаты в файл
        let scan_result = ScannerResult::new(chart, filter, marker, points);
        ScannerResult::save(&scan_result)
    }
}
