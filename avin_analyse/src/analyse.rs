/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::PathBuf;
use std::str::FromStr;

use cached::proc_macro::cached;
use polars::prelude::*;
use strum::IntoEnumIterator;

use avin_core::TimeFrame;
use avin_data::Iid;
use avin_utils::{AvinError, Cmd};

use crate::{Size, Sz};

pub trait Analyse {
    fn analyse(iid: &Iid, tf: TimeFrame) -> Result<(), AvinError>;
    fn analyse_all() -> Result<(), AvinError>;
    fn save(iid: &Iid, name: &str, df: &mut DataFrame) {
        let path = create_path(iid, name);
        Cmd::write_pqt(df, &path).unwrap();

        log::info!("Analyse save {}", path.display());
    }
    fn load(iid: &Iid, name: &str) -> Result<DataFrame, AvinError> {
        let path = create_path(iid, name);

        if !Cmd::is_exist(&path) {
            let msg = format!("analyse not found: {}", path.display());
            let err = AvinError::NotFound(msg);
            return Err(err);
        }

        cached_load_file(path)
    }
    fn delete(iid: &Iid, name: &str) -> Result<(), AvinError> {
        let mut path = iid.path();
        path.push("ANALYSE");
        path.push(name);

        if !Cmd::is_exist(&path) {
            log::info!("Skip delete {}", path.display());
            return Ok(());
        } else if path.is_dir() {
            Cmd::delete_dir(&path).unwrap();
            log::info!("Analyse delete {}", path.display());
        } else if path.is_file() {
            Cmd::delete(&path).unwrap();
            log::info!("Analyse delete {}", path.display());
        }

        Ok(())
    }

    fn eval_cdf(values: &Series) -> DataFrame {
        //! Return df where:
        //!    value - sorted
        //!    count - of this value
        //!    pf - probability function (= count / all_count)
        //!    cdf - cumulative distribution function
        //!    cdf_p - cumulative distribution function in %
        //! ┌───────┬───────┬──────────┬──────────┬───────────┐
        //! │ value ┆ count ┆ pf       ┆ cdf      ┆ cdf_p     │
        //! │ ---   ┆ ---   ┆ ---      ┆ ---      ┆ ---       │
        //! │ f64   ┆ u32   ┆ f64      ┆ f64      ┆ f64       │
        //! ╞═══════╪═══════╪══════════╪══════════╪═══════════╡
        //! │ 0.0   ┆ 2     ┆ 0.000102 ┆ 0.000102 ┆ 0.010154  │
        //! │ 0.02  ┆ 4     ┆ 0.000203 ┆ 0.000305 ┆ 0.030461  │
        //! │ 0.03  ┆ 4     ┆ 0.000203 ┆ 0.000508 ┆ 0.050769  │
        //! │ 0.04  ┆ 3     ┆ 0.000152 ┆ 0.00066  ┆ 0.066     │
        //! │ 0.05  ┆ 10    ┆ 0.000508 ┆ 0.001168 ┆ 0.116769  │
        //! │ …     ┆ …     ┆ …        ┆ …        ┆ …         │
        //! │ 24.71 ┆ 1     ┆ 0.000051 ┆ 0.999797 ┆ 99.979692 │
        //! │ 31.58 ┆ 1     ┆ 0.000051 ┆ 0.999848 ┆ 99.984769 │
        //! │ 51.82 ┆ 1     ┆ 0.000051 ┆ 0.999898 ┆ 99.989846 │
        //! │ 56.12 ┆ 1     ┆ 0.000051 ┆ 0.999949 ┆ 99.994923 │
        //! │ 65.05 ┆ 1     ┆ 0.000051 ┆ 1.0      ┆ 100.0     │
        //! └───────┴───────┴──────────┴──────────┴───────────┘

        // sort values
        let mut values = values.sort(SortOptions::default()).unwrap();
        values.set_sorted_flag(polars::series::IsSorted::Ascending);
        values.rename("value".into());

        // count
        let mut df = values
            .value_counts(false, false, "count".into(), false)
            .unwrap();

        // probability function - PF
        let pf = values
            .value_counts(false, false, "pf".into(), true)
            .unwrap()
            .column("pf")
            .unwrap()
            .as_materialized_series()
            .clone();

        // cumulative distribution function - CDF
        let cdf = cum_sum(&pf, false).unwrap().rename("cdf".into()).clone();

        // CDF in percent (* 100)
        let mut cdf_p = cdf.clone() * 100;
        cdf_p.rename("cdf_p".into());

        // join df
        df.with_column(pf).unwrap();
        df.with_column(cdf).unwrap();
        df.with_column(cdf_p).unwrap();

        df
    }
    fn eval_size(cdf: &DataFrame) -> DataFrame {
        let mut sizes = Vec::new();
        let mut begin = Vec::new();
        let mut end = Vec::new();

        // init first begin = first value in cdf dataframe
        let mut b = cdf
            .column("value")
            .unwrap()
            .as_materialized_series()
            .first()
            .value()
            .clone();
        // define variable for end value
        let mut e;

        // iterate all Size
        for size in Size::iter() {
            // filter where cdf_p <= size.range().max()
            let filtered = cdf
                .clone()
                .lazy()
                .filter(col("cdf_p").lt_eq(size.range().max()))
                .collect()
                .unwrap();

            // set end
            if filtered.is_empty() {
                // например первое значение = 2
                // и оно встречается сильно чаще чем в 1% случаев,
                // как в случае с period. Тогда там диапазоны
                // GreatestSmall, AnomalSmall, ExtraSmall, VerySmall...
                // все равны [2, 2]
                e = b.clone();
            } else {
                // end = last value in filtered
                e = filtered
                    .column("value")
                    .unwrap()
                    .as_materialized_series()
                    .last()
                    .value()
                    .clone()
            }

            // save name, begin, end
            sizes.push(size.name());
            begin.push(b.clone());
            end.push(e.clone());

            // set begin = end, for next iteration
            b = e.clone();
        }

        df!(
            "size" => sizes,
            "begin" => begin,
            "end" => end,
        )
        .unwrap()
    }
    fn eval_sz(cdf: &DataFrame) -> DataFrame {
        let mut sizes = Vec::new();
        let mut begin = Vec::new();
        let mut end = Vec::new();

        // init first begin = first value in cdf dataframe
        let mut b = cdf
            .column("value")
            .unwrap()
            .as_materialized_series()
            .first()
            .value()
            .clone();
        // define variable for end value
        let mut e;

        // iterate all Sz
        for size in Sz::iter() {
            // filter where cdf_p <= size.range().max()
            let filtered = cdf
                .clone()
                .lazy()
                .filter(col("cdf_p").lt_eq(size.range().max()))
                .collect()
                .unwrap();

            // set end
            if filtered.is_empty() {
                // например первое значение = 2
                // и оно встречается сильно чаще чем в 1% случаев,
                // как в случае с period. Тогда там диапазоны
                // GreatestSmall, AnomalSmall, ExtraSmall, VerySmall...
                // все равны [2, 2]
                e = b.clone();
            } else {
                // end = last value in filtered
                e = filtered
                    .column("value")
                    .unwrap()
                    .as_materialized_series()
                    .last()
                    .value()
                    .clone()
            }

            // save name, begin, end
            sizes.push(size.name());
            begin.push(b.clone());
            end.push(e.clone());

            // set begin = end, for next iteration
            b = e.clone();
        }

        df!(
            "sz" => sizes,
            "begin" => begin,
            "end" => end,
        )
        .unwrap()
    }

    fn cdf<T>(value: T, cdf_df: DataFrame) -> f64
    where
        T: NumericNative + Into<Expr>,
    {
        cdf_df
            .lazy()
            .filter(col("value").lt_eq(value))
            .last()
            .collect()
            .unwrap()
            .column("cdf")
            .unwrap()
            .f64()
            .unwrap()
            .last()
            .unwrap()
    }
    fn cdf_p<T>(value: T, cdf_df: DataFrame) -> f64
    where
        T: NumericNative + Into<Expr>,
    {
        cdf_df
            .lazy()
            .filter(col("value").lt_eq(value))
            .last()
            .collect()
            .unwrap()
            .column("cdf_p")
            .unwrap()
            .f64()
            .unwrap()
            .last()
            .unwrap()
    }
    fn size<T>(value: T, sizes: &DataFrame) -> Size
    where
        T: NumericNative + Into<Expr>,
    {
        let filtered = sizes
            .clone()
            .lazy()
            .filter(col("begin").lt_eq(value))
            .filter(col("end").gt(value))
            .collect()
            .unwrap();

        if filtered.height() == 1 {
            let name = filtered
                .column("size")
                .unwrap()
                .str()
                .unwrap()
                .last()
                .unwrap();
            let size = Size::from_str(name).unwrap();
            return size;
        }

        let greatest = sizes
            .column("end")
            .unwrap()
            .as_materialized_series()
            .max()
            .unwrap()
            .lt(&Some(value));

        if greatest {
            Size::GreatestBig
        } else {
            Size::GreatestSmall
        }
    }
    fn sz<T>(value: T, sizes: &DataFrame) -> Sz
    where
        T: NumericNative + Into<Expr>,
    {
        let filtered = sizes
            .clone()
            .lazy()
            .filter(col("begin").lt_eq(value))
            .filter(col("end").gt(value))
            .collect()
            .unwrap();

        if filtered.height() == 1 {
            let name = filtered
                .column("sz")
                .unwrap()
                .str()
                .unwrap()
                .last()
                .unwrap();
            let sz = Sz::from_str(name).unwrap();
            return sz;
        }

        let greatest = sizes
            .column("end")
            .unwrap()
            .as_materialized_series()
            .max()
            .unwrap()
            .lt(&Some(value));

        if greatest { Sz::XL } else { Sz::XS }
    }
}

// private
fn create_path(iid: &Iid, analyse_name: &str) -> PathBuf {
    let mut path = iid.path();

    path.push("ANALYSE");

    let analyse_name = format!("{analyse_name}.parquet");
    for part in analyse_name.split(' ') {
        path.push(part);
    }

    path
}
#[inline]
#[cached]
fn cached_load_file(path: PathBuf) -> Result<DataFrame, AvinError> {
    Cmd::read_pqt(&path)
}
