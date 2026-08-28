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

/// Describes the market data desired for the AVIN workspace.
///
/// The manifest is produced by parsing the workspace data.toml file and
/// contains one [DataProviderSet] for each configured market data source.
/// It represents the desired data state, but does not define whether that data
/// is downloaded, built from other data, or already present in storage.
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

    /// Returns the configured market data provider sets.
    pub fn sets(&self) -> &[DataProviderSet] {
        &self.sets
    }
}

/// Describes the market data desired from one source.
///
/// Empty collections mean that the corresponding data is not requested.
/// A history depth of `0` means that no history of that type is requested.
#[derive(Debug)]
pub struct DataProviderSet {
    /// Market data source.
    pub source: Source,
    /// Instruments for which data is desired.
    pub instruments: Vec<InstrumentId>,

    /// Number of years of bar history desired.
    pub bar_history_years: u32,
    /// Bar timeframes desired.
    pub bar_timeframes: Vec<TimeFrame>,

    /// Number of years of tick history desired.
    pub tick_history_years: u32,

    /// Time-based footprint intervals desired.
    pub time_footprints: Vec<TimeFrame>,
    /// Tick-based footprint sizes desired.
    pub tick_footprints: Vec<u64>,
    /// Volume-based footprint sizes desired.
    pub volume_footprints: Vec<u64>,
    /// Value-based footprint sizes desired.
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

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    fn data_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn read_empty_manifest() {
        let file = data_file(
            r#"
format = 1
"#,
        );

        let manifest = DataManifest::read(file.path()).unwrap();

        assert!(manifest.sets().is_empty());
    }

    #[test]
    fn read_empty_source() {
        let file = data_file(
            r#"
format = 1

[tbank]
"#,
        );

        let manifest = DataManifest::read(file.path()).unwrap();

        assert_eq!(manifest.sets().len(), 1);

        let set = &manifest.sets()[0];

        assert_eq!(set.source, Source::TBank);
        assert!(set.instruments.is_empty());

        assert_eq!(set.bar_history_years, 0);
        assert!(set.bar_timeframes.is_empty());

        assert_eq!(set.tick_history_years, 0);

        assert!(set.time_footprints.is_empty());
        assert!(set.tick_footprints.is_empty());
        assert!(set.volume_footprints.is_empty());
        assert!(set.value_footprints.is_empty());
    }

    #[test]
    fn read_full_source() {
        let file = data_file(
            r#"
format = 1

[tbank]
instruments = [
    "MOEX.SHARE.GAZP",
    "MOEX.SHARE.SBER",
]

[tbank.bars]
history_years = 5
timeframes = ["1M", "5M", "1H"]

[tbank.ticks]
history_years = 3

[tbank.footprints]
time = ["1M", "5M"]
tick = [100, 500]
volume = [1000, 5000]
value = [100_000, 1_000_000]
"#,
        );

        let manifest = DataManifest::read(file.path()).unwrap();

        assert_eq!(manifest.sets().len(), 1);

        let set = &manifest.sets()[0];

        assert_eq!(set.source, Source::TBank);

        assert_eq!(
            set.instruments,
            vec![
                InstrumentId::from_str("MOEX.SHARE.GAZP").unwrap(),
                InstrumentId::from_str("MOEX.SHARE.SBER").unwrap(),
            ]
        );

        assert_eq!(set.bar_history_years, 5);
        assert_eq!(
            set.bar_timeframes,
            vec![
                TimeFrame::from_str("1M").unwrap(),
                TimeFrame::from_str("5M").unwrap(),
                TimeFrame::from_str("1H").unwrap(),
            ]
        );

        assert_eq!(set.tick_history_years, 3);

        assert_eq!(
            set.time_footprints,
            vec![
                TimeFrame::from_str("1M").unwrap(),
                TimeFrame::from_str("5M").unwrap(),
            ]
        );
        assert_eq!(set.tick_footprints, vec![100, 500]);
        assert_eq!(set.volume_footprints, vec![1000, 5000]);
        assert_eq!(set.value_footprints, vec![100_000, 1_000_000]);
    }

    #[test]
    fn read_zero_and_empty_bar_data() {
        let file = data_file(
            r#"
format = 1

[tbank.bars]
history_years = 0
timeframes = []
"#,
        );

        let manifest = DataManifest::read(file.path()).unwrap();
        let set = &manifest.sets()[0];

        assert_eq!(set.bar_history_years, 0);
        assert!(set.bar_timeframes.is_empty());
    }

    #[test]
    fn read_empty_footprints() {
        let file = data_file(
            r#"
format = 1

[tbank.footprints]
"#,
        );

        let manifest = DataManifest::read(file.path()).unwrap();
        let set = &manifest.sets()[0];

        assert!(set.time_footprints.is_empty());
        assert!(set.tick_footprints.is_empty());
        assert!(set.volume_footprints.is_empty());
        assert!(set.value_footprints.is_empty());
    }

    #[test]
    fn reject_unknown_root_field() {
        let file = data_file(
            r#"
format = 1

[ebanina]
foo = 42
"#,
        );

        assert!(DataManifest::read(file.path()).is_err());
    }

    #[test]
    fn reject_unknown_source_field() {
        let file = data_file(
            r#"
format = 1

[tbank]
ebanina = ["MOEX.SHARE.SBER"]
"#,
        );

        assert!(DataManifest::read(file.path()).is_err());
    }

    #[test]
    fn reject_missing_bar_field() {
        let file = data_file(
            r#"
format = 1

[tbank.bars]
history_years = 2
"#,
        );

        assert!(DataManifest::read(file.path()).is_err());
    }

    #[test]
    fn reject_missing_tick_field() {
        let file = data_file(
            r#"
format = 1

[tbank.ticks]
"#,
        );

        assert!(DataManifest::read(file.path()).is_err());
    }

    #[test]
    fn reject_invalid_instrument() {
        let file = data_file(
            r#"
format = 1

[tbank]
instruments = ["invalid"]
"#,
        );

        assert!(DataManifest::read(file.path()).is_err());
    }

    #[test]
    fn reject_invalid_bar_timeframe() {
        let file = data_file(
            r#"
format = 1

[tbank.bars]
history_years = 2
timeframes = ["1M", "EBANINA"]
"#,
        );

        assert!(DataManifest::read(file.path()).is_err());
    }

    #[test]
    fn reject_invalid_time_footprint() {
        let file = data_file(
            r#"
format = 1

[tbank.footprints]
time = ["1M", "EBANINA"]
"#,
        );

        assert!(DataManifest::read(file.path()).is_err());
    }

    #[test]
    fn reject_missing_format() {
        let file = data_file(
            r#"
[tbank]
"#,
        );

        assert!(DataManifest::read(file.path()).is_err());
    }
}
