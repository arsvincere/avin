// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::cmp::{max, min};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::ops::RangeInclusive;
use std::path::Path;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Datelike, Utc};
use ureq::{Agent, Error as HttpError};
use zip::ZipArchive;

use avin_core::{Price, Quantity, Time, TimeRange};
use avin_domain::{Bar, InstrumentInfo, TimeFrame};
use avin_system::Workspace;

use crate::{BarsPack, DataError};

pub(super) struct TBankBarsIterator {
    instrument: InstrumentInfo,
    tf: TimeFrame,
    range: TimeRange,
    uid: String,
    token: String,
    agent: Agent,
    years: RangeInclusive<i32>,
    archive: Option<YearArchive>,
}

struct YearArchive {
    zip: ZipArchive<File>,
    entries: std::vec::IntoIter<usize>,
}

impl TBankBarsIterator {
    pub(super) fn new(
        instrument: InstrumentInfo,
        tf: TimeFrame,
        range: TimeRange,
    ) -> Result<Self, DataError> {
        let uid = instrument
            .raw_info()
            .get("uid")
            .cloned()
            .ok_or_else(|| {
                let msg = format!(
                    "T-Bank instrument {} has no uid",
                    instrument.iid(),
                );
                DataError::unavailable(msg, None)
            })?;

        let ws = Workspace::get().map_err(|err| {
            connect_error("failed to access workspace", err)
        })?;

        let first_year = range.begin().dt().year();
        let last_year = Time::new(range.end().ts() - 1).dt().year();

        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .build();

        Ok(Self {
            instrument,
            tf,
            range,
            uid,
            token: ws.secret.tbank_token().to_string(),
            agent: Agent::new_with_config(config),
            years: first_year..=last_year,
            archive: None,
        })
    }

    fn download_year(&self, year: i32) -> Result<Option<YearArchive>, DataError> {
        let mut file = tempfile::tempfile().map_err(|err| {
            connect_error("failed to create temp file for T-Bank bars", err)
        })?;

        if !download_archive(
            &self.agent,
            &self.token,
            &self.uid,
            year,
            &mut file,
        )? {
            return Ok(None);
        }

        file.seek(SeekFrom::Start(0)).map_err(|err| {
            connect_error("failed to rewind T-Bank archive", err)
        })?;

        YearArchive::new(file).map(Some)
    }

    fn next_pack(&mut self) -> Option<Result<BarsPack, DataError>> {
        loop {
            let archive = self.archive.as_mut()?;
            let index = archive.entries.next()?;

            let file = match archive.zip.by_index(index) {
                Ok(file) => file,
                Err(err) => {
                    return Some(Err(connect_error(
                        "failed to read T-Bank archive entry",
                        err,
                    )));
                }
            };
            let name = file.name().to_string();

            match read_pack(
                &self.instrument,
                self.tf,
                self.range,
                &self.uid,
                &name,
                file,
            ) {
                Ok(Some(pack)) => return Some(Ok(pack)),
                Ok(None) => continue,
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

impl Iterator for TBankBarsIterator {
    type Item = Result<BarsPack, DataError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(result) = self.next_pack() {
                return Some(result);
            }

            let year = self.years.next()?;

            match self.download_year(year) {
                Ok(Some(archive)) => self.archive = Some(archive),
                Ok(None) => continue,
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

impl YearArchive {
    fn new(file: File) -> Result<Self, DataError> {
        let mut zip = ZipArchive::new(file).map_err(|err| {
            connect_error("failed to open T-Bank ZIP archive", err)
        })?;

        let mut entries = Vec::new();
        for index in 0..zip.len() {
            let entry = zip.by_index(index).map_err(|err| {
                connect_error("failed to inspect T-Bank ZIP archive", err)
            })?;

            if !entry.is_dir() {
                entries.push((entry.name().to_string(), index));
            }
        }

        entries.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(Self {
            zip,
            entries: entries
                .into_iter()
                .map(|(_, index)| index)
                .collect::<Vec<_>>()
                .into_iter(),
        })
    }
}

fn download_archive(
    agent: &Agent,
    token: &str,
    uid: &str,
    year: i32,
    file: &mut File,
) -> Result<bool, DataError> {
    let url = format!(
        "https://invest-public-api.tbank.ru/history-data?instrumentId={uid}&year={year}"
    );
    let authorization = format!("Bearer {token}");

    for attempt in 0..5 {
        let response = agent
            .get(&url)
            .header("Authorization", authorization.as_str())
            .call();

        match response {
            Ok(mut response) => {
                let size = io::copy(
                    &mut response.body_mut().as_reader(),
                    file,
                )
                .map_err(|err| {
                    connect_error(
                        format!("failed to download T-Bank bars for {year}"),
                        err,
                    )
                })?;

                return Ok(size > 0);
            }

            Err(HttpError::StatusCode(404)) => return Ok(false),

            Err(HttpError::StatusCode(code)) if !retryable(code) => {
                let msg = format!(
                    "failed to download T-Bank bars for {year}: HTTP {code}"
                );
                return Err(DataError::connect(msg, None));
            }

            Err(err) => {
                if attempt == 4 {
                    let msg = format!(
                        "failed to download T-Bank bars for {year} after 5 attempts"
                    );
                    return Err(connect_error(msg, err));
                }

                thread::sleep(Duration::from_secs(1_u64 << attempt));
            }
        }
    }

    unreachable!()
}

fn retryable(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
}

fn read_pack(
    instrument: &InstrumentInfo,
    tf: TimeFrame,
    requested: TimeRange,
    uid: &str,
    name: &str,
    reader: impl Read,
) -> Result<Option<BarsPack>, DataError> {
    let Some(day_begin) = parse_file_day(name, uid)? else {
        return Ok(None);
    };
    let day_end = TimeFrame::Day.end_frame(day_begin);

    if day_end <= requested.begin() || day_begin >= requested.end() {
        return Ok(None);
    }

    let begin = max(day_begin, requested.begin());
    let end = min(day_end, requested.end());
    let range = TimeRange::new(begin, end).unwrap();

    let mut bars = Vec::new();
    for (index, line) in BufReader::new(reader).lines().enumerate() {
        let line = line.map_err(|err| {
            connect_error(format!("failed to read {name}:{}", index + 1), err)
        })?;

        if line.is_empty() {
            continue;
        }

        let bar = parse_bar(&line, uid, name, index + 1)?;
        if range.contains(bar.time) {
            bars.push(bar);
        }
    }

    if bars.is_empty() {
        return Ok(None);
    }

    bars.sort_by_key(|bar| bar.time);

    BarsPack::new(instrument.clone(), tf, range, bars).map(Some)
}

fn parse_file_day(name: &str, uid: &str) -> Result<Option<Time>, DataError> {
    let Some(name) = Path::new(name)
        .file_name()
        .and_then(|name| name.to_str())
    else {
        return Ok(None);
    };

    let prefix = format!("{uid}_");
    let Some(raw) = name
        .strip_prefix(&prefix)
        .and_then(|name| name.strip_suffix(".csv"))
    else {
        return Ok(None);
    };

    if raw.len() != 8 || !raw.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(None);
    }

    let date = format!("{}-{}-{}", &raw[..4], &raw[4..6], &raw[6..]);
    let time = Time::from_str(&date).map_err(|err| {
        pack_error(format!("invalid T-Bank bar file '{name}'"), err)
    })?;

    Ok(Some(time))
}

fn parse_bar(
    line: &str,
    uid: &str,
    file: &str,
    line_number: usize,
) -> Result<Bar, DataError> {
    let fields: Vec<&str> = line.split(';').collect();
    if fields.len() < 7 {
        return Err(row_error(file, line_number, "expected 7 fields"));
    }

    if fields[0] != uid {
        return Err(row_error(
            file,
            line_number,
            &format!("unexpected uid '{}'", fields[0]),
        ));
    }

    let dt = DateTime::parse_from_rfc3339(fields[1]).map_err(|err| {
        row_error_source(
            file,
            line_number,
            &format!("invalid time '{}'", fields[1]),
            err,
        )
    })?;
    let time = Time::try_from(dt.with_timezone(&Utc)).map_err(|err| {
        row_error_source(file, line_number, "invalid time", err)
    })?;

    let open = price(fields[2], "open", file, line_number)?;
    let close = price(fields[3], "close", file, line_number)?;
    let high = price(fields[4], "high", file, line_number)?;
    let low = price(fields[5], "low", file, line_number)?;
    let volume = quantity(fields[6], "volume", file, line_number)?;

    Bar::new(time, open, high, low, close, volume).map_err(|err| {
        row_error_source(file, line_number, "invalid OHLCV", err)
    })
}

fn price(
    value: &str,
    field: &str,
    file: &str,
    line: usize,
) -> Result<Price, DataError> {
    let value = number(value, field, file, line)?;
    Price::new(value).map_err(|err| row_error_source(file, line, field, err))
}

fn quantity(
    value: &str,
    field: &str,
    file: &str,
    line: usize,
) -> Result<Quantity, DataError> {
    let value = number(value, field, file, line)?;
    Quantity::new(value)
        .map_err(|err| row_error_source(file, line, field, err))
}

fn number(
    value: &str,
    field: &str,
    file: &str,
    line: usize,
) -> Result<f64, DataError> {
    value.parse::<f64>().map_err(|err| {
        row_error_source(
            file,
            line,
            &format!("invalid {field} '{value}'"),
            err,
        )
    })
}

fn row_error(file: &str, line: usize, message: &str) -> DataError {
    DataError::pack(format!("{file}:{line}: {message}"), None)
}

fn row_error_source(
    file: &str,
    line: usize,
    message: &str,
    source: impl Error + Send + Sync + 'static,
) -> DataError {
    pack_error(format!("{file}:{line}: {message}"), source)
}

fn connect_error(
    message: impl Into<String>,
    source: impl Error + Send + Sync + 'static,
) -> DataError {
    DataError::connect(message, Some(Box::new(source)))
}

fn pack_error(
    message: impl Into<String>,
    source: impl Error + Send + Sync + 'static,
) -> DataError {
    DataError::pack(message, Some(Box::new(source)))
}
