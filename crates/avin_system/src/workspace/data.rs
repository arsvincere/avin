// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: del after impl
#![allow(dead_code)]
#![allow(unused)]

use std::path::Path;

use serde::Deserialize;

use avin_core::Source;
use avin_domain::{InstrumentId, TimeFrame};
use avin_utils::AvinError;

#[derive(Debug)]
pub struct DataManifest {
    sets: Vec<DataProviderSet>,
}

impl DataManifest {
    pub(super) fn read(path: &Path) -> Result<Self, AvinError> {
        let raw: DataToml = avin_utils::read_toml(path)?;

        let mut sets = Vec::new();

        if let Some(tbank) = raw.tbank {
            let tbank_set = DataProviderSet {
                source: Source::TBank,
                instruments: get_instruments(&tbank)?,
                bar_history_years: get_bar_history_years(&tbank)?,
                bar_timeframes: get_bar_timeframes(&tbank)?,
                tick_history_years: get_tick_history_years(&tbank)?,
                time_footprint: get_time_footprint(&tbank)?,
                tick_footprint: get_tick_footprint(&tbank)?,
                volume_footprint: get_volume_footprint(&tbank)?,
                value_footprint: get_value_footprint(&tbank)?,
            };
            sets.push(tbank_set);
        }

        Ok(Self { sets })
    }

    pub fn sets(&self) -> &[DataProviderSet] {
        &self.sets
    }
}

#[derive(Debug)]
pub struct DataProviderSet {
    pub source: Source,
    pub instruments: Vec<InstrumentId>,

    pub bar_history_years: u32,
    pub bar_timeframes: Vec<TimeFrame>,

    pub tick_history_years: u32,

    pub time_footprint: Vec<TimeFrame>,
    pub tick_footprint: Vec<u64>,
    pub volume_footprint: Vec<u64>,
    pub value_footprint: Vec<u64>,
}

// TOML ---------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DataToml {
    #[allow(dead_code)]
    format: u32,
    tbank: Option<SourceDataToml>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceDataToml {
    instruments: Option<Vec<String>>,
    bars: Option<BarsDataToml>,
    ticks: Option<TicksDataToml>,
    footprints: Option<FootprintsDataToml>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BarsDataToml {
    history_years: u32,
    timeframes: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TicksDataToml {
    history_years: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FootprintsDataToml {
    time: Option<Vec<String>>,
    tick: Option<Vec<u64>>,
    volume: Option<Vec<u64>>,
    value: Option<Vec<u64>>,
}

// helpers -------------------------------------------------------------------

fn get_instruments(
    source: &SourceDataToml,
) -> Result<Vec<InstrumentId>, AvinError> {
    todo!()
}

fn get_bar_history_years(source: &SourceDataToml) -> Result<u32, AvinError> {
    todo!()
}

fn get_bar_timeframes(
    source: &SourceDataToml,
) -> Result<Vec<TimeFrame>, AvinError> {
    todo!()
}

fn get_tick_history_years(source: &SourceDataToml) -> Result<u32, AvinError> {
    todo!()
}

fn get_time_footprint(
    source: &SourceDataToml,
) -> Result<Vec<TimeFrame>, AvinError> {
    todo!()
}

fn get_tick_footprint(
    source: &SourceDataToml,
) -> Result<Vec<u64>, AvinError> {
    todo!()
}

fn get_volume_footprint(
    source: &SourceDataToml,
) -> Result<Vec<u64>, AvinError> {
    todo!()
}

fn get_value_footprint(
    source: &SourceDataToml,
) -> Result<Vec<u64>, AvinError> {
    todo!()
}
