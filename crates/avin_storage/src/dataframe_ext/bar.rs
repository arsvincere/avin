// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_core::{Price, Quantity, Time};
use polars::prelude::{Column, DataFrame};

use avin_domain::Bar;

use crate::StorageError;

use super::DataFrameExt;
use super::schema::StorageSchema;

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

        // schema is checked, then unwrap() is ok
        let time = df.column("timestamp").unwrap().i64().unwrap();
        let open = df.column("open").unwrap().f64().unwrap();
        let high = df.column("high").unwrap().f64().unwrap();
        let low = df.column("low").unwrap().f64().unwrap();
        let close = df.column("close").unwrap().f64().unwrap();
        let volume = df.column("volume").unwrap().f64().unwrap();

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
