// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::{BTreeSet, HashMap};

use polars::prelude::{Column, DataFrame};

use avin_domain::InstrumentInfo;

use crate::StorageError;

/// Conversion between AVIN domain objects and Polars DataFrames.
pub trait DataFrameExt: Sized {
    /// Converts a slice of domain objects into a DataFrame.
    fn to_df(data: &[Self]) -> Result<DataFrame, StorageError>;

    /// Converts a DataFrame into domain objects.
    fn from_df(df: DataFrame) -> Result<Vec<Self>, StorageError>;
}

impl DataFrameExt for InstrumentInfo {
    fn to_df(instruments: &[Self]) -> Result<DataFrame, StorageError> {
        if instruments.is_empty() {
            return Err(StorageError::conversion(
                "no instruments to convert",
                None,
            ));
        }

        // collect all unique column names
        let mut names = BTreeSet::new();
        for instrument in instruments.iter() {
            names.extend(instrument.raw_info().keys().cloned());
        }

        // identification columns first
        let mut ordered = Vec::with_capacity(names.len());
        for key in ["provider", "exchange", "category", "ticker", "name"] {
            if let Some(key) = names.take(key) {
                ordered.push(key);
            }
        }

        // remaining columns alphabetically
        ordered.extend(names);

        // create columns
        let mut columns = Vec::with_capacity(ordered.len());

        for key in ordered {
            let mut values = Vec::with_capacity(instruments.len());

            for instrument in instruments {
                let value = instrument
                    .raw_info()
                    .get(&key)
                    .map(|value| value.as_str());

                values.push(value);
            }

            let column = Column::new(key.into(), values);

            columns.push(column);
        }

        // create DataFrame
        DataFrame::new(instruments.len(), columns).map_err(|err| {
            let msg = "failed to create instruments DataFrame";
            StorageError::conversion(msg, Some(err.into()))
        })
    }

    fn from_df(df: DataFrame) -> Result<Vec<Self>, StorageError> {
        let mut instruments = Vec::with_capacity(df.height());

        // create instruments
        for row in 0..df.height() {
            let mut info = HashMap::with_capacity(df.width());

            // read instrument fields
            for column in df.columns() {
                let key = column.name();

                let values = column.str().map_err(|err| {
                    let msg = format!("column '{key}' must contain strings");
                    StorageError::conversion(msg, Some(err.into()))
                })?;

                if let Some(value) = values.get(row) {
                    info.insert(key.to_string(), value.to_string());
                }
            }

            // create instrument
            let instrument = InstrumentInfo::new_unchecked(info);
            instruments.push(instrument);
        }

        Ok(instruments)
    }
}
