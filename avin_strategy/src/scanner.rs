/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused)]

use serde::{Deserialize, Serialize};

use avin_analyse::TrendAnalytic;
use avin_core::Term::T1;
use avin_core::{Asset, Chart, ExtremumIndicator, Iid, Manager, TimeFrame};
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
    S = 5,
    M = 8,
    L = 13,
}
impl std::fmt::Display for MarkerSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S => write!(f, "S"),
            Self::M => write!(f, "M"),
            Self::L => write!(f, "L"),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Marker {
    shape: MarkerShape,
    color: MarkerColor,
    size: MarkerSize,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ScannerResult {
    scan_name: String,
    iid_name: String,
    tf: TimeFrame,
    marker: Marker,
    points: Vec<i64>,
}
impl ScannerResult {
    fn new(
        chart: &Chart,
        filter: impl Filter,
        marker: Marker,
        points: Vec<i64>,
    ) -> Self {
        Self {
            scan_name: format!("{}_{}", filter.name(), chart.ticker()),
            iid_name: chart.iid().to_string(),
            tf: chart.tf(),
            marker,
            points,
        }
    }

    fn save(result: &ScannerResult) -> Result<(), AvinError> {
        let text = toml::to_string_pretty(result).unwrap();
        let mut path = CFG.dir.scan();
        path.push(format!("{}.toml", result.scan_name));

        Cmd::write(&text, &path).unwrap();

        Ok(())
    }
    fn load_name(name: &str) -> Result<Self, AvinError> {
        let mut path = CFG.dir.scan();
        path.push(format!("{name}.toml"));

        let text = Cmd::read(&path).unwrap();
        let r: ScannerResult = toml::from_str(&text).unwrap();

        Ok(r)
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
                let point = new_chart.now().unwrap().ts_nanos;
                points.push(point);
            }
        }

        // сохраняем результаты в файл
        let scan_result = ScannerResult::new(chart, filter, marker, points);
        ScannerResult::save(&scan_result)
    }
}
