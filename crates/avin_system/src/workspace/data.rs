// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;
use std::str::FromStr;

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
                bar_history_years: get_bar_history_years(&tbank),
                bar_timeframes: get_bar_timeframes(&tbank)?,
                tick_history_years: get_tick_history_years(&tbank),
                time_footprints: get_time_footprints(&tbank)?,
                tick_footprints: get_tick_footprints(&tbank),
                volume_footprints: get_volume_footprints(&tbank),
                value_footprints: get_value_footprints(&tbank),
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

    pub time_footprints: Vec<TimeFrame>,
    pub tick_footprints: Vec<u64>,
    pub volume_footprints: Vec<u64>,
    pub value_footprints: Vec<u64>,
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
    let instruments = match &source.instruments {
        Some(instruments) => instruments,
        None => return Ok(Vec::new()),
    };

    let mut result = Vec::new();

    for iid_str in instruments.iter() {
        let iid = InstrumentId::from_str(iid_str)?;
        result.push(iid);
    }

    Ok(result)
}

fn get_bar_history_years(source: &SourceDataToml) -> u32 {
    match &source.bars {
        None => 0,
        Some(bars_data) => bars_data.history_years,
    }
}

fn get_bar_timeframes(
    source: &SourceDataToml,
) -> Result<Vec<TimeFrame>, AvinError> {
    let bars_data = match &source.bars {
        None => return Ok(Vec::new()),
        Some(bars_data) => bars_data,
    };

    let mut timeframes = Vec::new();

    for tf_str in bars_data.timeframes.iter() {
        let tf = TimeFrame::from_str(tf_str)?;
        timeframes.push(tf);
    }

    Ok(timeframes)
}

fn get_tick_history_years(source: &SourceDataToml) -> u32 {
    match &source.ticks {
        None => 0,
        Some(ticks_data) => ticks_data.history_years,
    }
}

fn get_time_footprints(
    source: &SourceDataToml,
) -> Result<Vec<TimeFrame>, AvinError> {
    let footprints_data = match &source.footprints {
        None => return Ok(Vec::new()),
        Some(footprints_data) => footprints_data,
    };

    let time_footprints = match &footprints_data.time {
        None => return Ok(Vec::new()),
        Some(time_footprints) => time_footprints,
    };

    let mut timeframes = Vec::new();

    for tf_str in time_footprints.iter() {
        let tf = TimeFrame::from_str(tf_str)?;
        timeframes.push(tf);
    }

    Ok(timeframes)
}

fn get_tick_footprints(source: &SourceDataToml) -> Vec<u64> {
    let footprints_data = match &source.footprints {
        None => return Vec::new(),
        Some(footprints_data) => footprints_data,
    };

    match &footprints_data.tick {
        None => Vec::new(),
        Some(tick_footprints) => tick_footprints.clone(),
    }
}

fn get_volume_footprints(source: &SourceDataToml) -> Vec<u64> {
    let footprints_data = match &source.footprints {
        None => return Vec::new(),
        Some(footprints_data) => footprints_data,
    };

    match &footprints_data.volume {
        None => Vec::new(),
        Some(volume_footprints) => volume_footprints.clone(),
    }
}

fn get_value_footprints(source: &SourceDataToml) -> Vec<u64> {
    let footprints_data = match &source.footprints {
        None => return Vec::new(),
        Some(footprints_data) => footprints_data,
    };

    match &footprints_data.value {
        None => Vec::new(),
        Some(value_footprints) => value_footprints.clone(),
    }
}
