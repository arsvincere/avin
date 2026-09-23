// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::{BTreeSet, HashMap};

use avin_core::{Price, Quantity, Time};
use polars::prelude::{Column, DataFrame};

use avin_domain::{Bar, InstrumentInfo};

use crate::{StorageError, StorageSchema};

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
            let msg = "no instruments to convert";
            return Err(StorageError::conversion(msg, None));
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

impl DataFrameExt for Bar {
    fn to_df(bars: &[Self]) -> Result<DataFrame, StorageError> {
        let dt = bars.iter().map(|b| b.time.to_string()).collect::<Vec<_>>();
        let open = bars.iter().map(|b| b.o.value()).collect::<Vec<_>>();
        let high = bars.iter().map(|b| b.h.value()).collect::<Vec<_>>();
        let low = bars.iter().map(|b| b.l.value()).collect::<Vec<_>>();
        let close = bars.iter().map(|b| b.c.value()).collect::<Vec<_>>();
        let volume = bars.iter().map(|b| b.v.value()).collect::<Vec<_>>();
        let ts = bars.iter().map(|b| b.time.ts()).collect::<Vec<_>>();

        let columns = vec![
            Column::new("datetime".into(), dt),
            Column::new("open".into(), open),
            Column::new("high".into(), high),
            Column::new("low".into(), low),
            Column::new("close".into(), close),
            Column::new("volume".into(), volume),
            Column::new("timestamp".into(), ts),
        ];

        DataFrame::new(bars.len(), columns).map_err(|err| {
            let msg = "failed to create bars DataFrame";
            StorageError::conversion(msg, Some(err.into()))
        })
    }

    fn from_df(df: DataFrame) -> Result<Vec<Self>, StorageError> {
        if df.schema().as_ref() != &StorageSchema::bar() {
            let msg = "invalid bars DataFrame schema";
            return Err(StorageError::conversion(msg, None));
        }

        let time = df.column("timestamp").unwrap().i64().map_err(|err| {
            let msg = "column 'timestamp' must contain i64";
            StorageError::conversion(msg, Some(err.into()))
        })?;

        let open = df.column("open").unwrap().f64().map_err(|err| {
            let msg = "column 'open' must contain f64";
            StorageError::conversion(msg, Some(err.into()))
        })?;

        let high = df.column("high").unwrap().f64().map_err(|err| {
            let msg = "column 'high' must contain f64";
            StorageError::conversion(msg, Some(err.into()))
        })?;

        let low = df.column("low").unwrap().f64().map_err(|err| {
            let msg = "column 'low' must contain f64";
            StorageError::conversion(msg, Some(err.into()))
        })?;

        let close = df.column("close").unwrap().f64().map_err(|err| {
            let msg = "column 'close' must contain f64";
            StorageError::conversion(msg, Some(err.into()))
        })?;

        let volume = df
            .column("volume")
            .and_then(|column| column.f64())
            .map_err(|err| {
                let msg = "column 'volume' must contain f64";
                StorageError::conversion(msg, Some(err.into()))
            })?;

        let mut bars = Vec::with_capacity(df.height());

        for row in 0..df.height() {
            let t = time.get(row).expect("must be not null");
            let o = open.get(row).expect("must be not null");
            let h = high.get(row).expect("must be not null");
            let l = low.get(row).expect("must be not null");
            let c = close.get(row).expect("must be not null");
            let v = volume.get(row).expect("must be not null");

            let t = Time::new(t);
            let o = Price::new(o).expect("must be valid");
            let h = Price::new(h).expect("must be valid");
            let l = Price::new(l).expect("must be valid");
            let c = Price::new(c).expect("must be valid");
            let v = Quantity::new(v).expect("must be valid");

            let bar = Bar::new_unchecked(t, o, h, l, c, v);

            bars.push(bar);
        }

        Ok(bars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn instrument(
        ticker: &str,
        extra: Option<(&str, &str)>,
    ) -> InstrumentInfo {
        let mut info = HashMap::from([
            ("provider".to_string(), "tbank".to_string()),
            ("exchange".to_string(), "moex".to_string()),
            ("category".to_string(), "share".to_string()),
            ("ticker".to_string(), ticker.to_string()),
            ("name".to_string(), ticker.to_string()),
            ("price_step".to_string(), "0.01".to_string()),
            ("lot_size".to_string(), "10".to_string()),
        ]);

        if let Some((key, value)) = extra {
            info.insert(key.to_string(), value.to_string());
        }

        InstrumentInfo::new_unchecked(info)
    }

    #[test]
    fn to_df() {
        let instruments = vec![
            instrument("SBER", Some(("z_extra", "foo"))),
            instrument("GAZP", Some(("a_extra", "bar"))),
        ];

        let df = InstrumentInfo::to_df(&instruments).unwrap();

        // check dimensions
        assert_eq!(df.height(), 2);
        assert_eq!(df.width(), 9);

        // check column order
        let names = df
            .columns()
            .iter()
            .map(|column| column.name().as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            [
                "provider",
                "exchange",
                "category",
                "ticker",
                "name",
                "a_extra",
                "lot_size",
                "price_step",
                "z_extra",
            ]
        );

        // check values and nulls
        let tickers = df.column("ticker").unwrap().str().unwrap();
        assert_eq!(tickers.get(0), Some("SBER"));
        assert_eq!(tickers.get(1), Some("GAZP"));

        let extra = df.column("z_extra").unwrap().str().unwrap();
        assert_eq!(extra.get(0), Some("foo"));
        assert_eq!(extra.get(1), None);
    }

    #[test]
    fn to_df_empty() {
        let result = InstrumentInfo::to_df(&[]);

        assert!(matches!(result, Err(StorageError::Conversion { .. })));
    }

    #[test]
    fn from_df() {
        let original = vec![
            instrument("SBER", Some(("uid", "SBER_UID"))),
            instrument("GAZP", None),
        ];

        let df = InstrumentInfo::to_df(&original).unwrap();
        let restored = InstrumentInfo::from_df(df).unwrap();

        assert_eq!(restored.len(), original.len());

        for (first, second) in original.iter().zip(restored.iter()) {
            assert_eq!(first.raw_info(), second.raw_info());
        }
    }

    #[test]
    fn from_df_invalid_column() {
        let columns = vec![Column::new("ticker".into(), vec![1_i32, 2_i32])];

        let df = DataFrame::new(2, columns).unwrap();

        let result = InstrumentInfo::from_df(df);

        assert!(matches!(result, Err(StorageError::Conversion { .. })));
    }
}
