// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;

use serde::Deserialize;

use avin_domain::{InstrumentId, TimeFrame};
use avin_utils::AvinError;

#[derive(Debug, Deserialize)]
pub struct Data {
    #[allow(dead_code)]
    format: u32,
    pub tbank: DataTBank,
}

impl Data {
    pub(super) fn read(path: &Path) -> Result<Self, AvinError> {
        let data: Self = avin_utils::read_toml(path)?;

        data.validate()?;

        Ok(data)
    }

    fn validate(&self) -> Result<(), AvinError> {
        for value in &self.tbank.instruments {
            value.parse::<InstrumentId>()?;
        }

        for value in &self.tbank.bars.timeframes {
            value.parse::<TimeFrame>()?;
        }

        for value in &self.tbank.footprints.time {
            value.parse::<TimeFrame>()?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct DataTBank {
    instruments: Vec<String>,
    pub bars: DataBars,
    pub ticks: DataTicks,
    pub footprints: DataFootprints,
}

impl DataTBank {
    pub fn instruments(&self) -> Vec<InstrumentId> {
        self.instruments
            .iter()
            .map(|value| value.parse().unwrap())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct DataBars {
    history_years: usize,
    timeframes: Vec<String>,
}

impl DataBars {
    pub fn history_years(&self) -> usize {
        self.history_years
    }

    pub fn timeframes(&self) -> Vec<TimeFrame> {
        self.timeframes
            .iter()
            .map(|value| value.parse().unwrap())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct DataTicks {
    history_years: usize,
}

impl DataTicks {
    pub fn history_years(&self) -> usize {
        self.history_years
    }
}

#[derive(Debug, Deserialize)]
pub struct DataFootprints {
    time: Vec<String>,
    tick: Vec<usize>,
    volume: Vec<usize>,
    value: Vec<usize>,
}

impl DataFootprints {
    pub fn time(&self) -> Vec<TimeFrame> {
        self.time
            .iter()
            .map(|value| value.parse().unwrap())
            .collect()
    }

    pub fn tick(&self) -> &[usize] {
        &self.tick
    }

    pub fn volume(&self) -> &[usize] {
        &self.volume
    }

    pub fn value(&self) -> &[usize] {
        &self.value
    }
}
