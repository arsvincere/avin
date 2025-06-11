/*****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::PathBuf;
use std::str::FromStr;

use cached::proc_macro::cached;
use polars::prelude::*;
use strum::IntoEnumIterator;

use crate::{Cmd, IID, Size, Sz, TimeFrame};

pub trait Analytic {
    fn name() -> &'static str;
    fn analyse(iid: &IID, tf: &TimeFrame) -> Result<(), String>;
    fn analyse_all() -> Result<(), String>;
    fn save(iid: &IID, name: &str, df: &mut DataFrame) {
        let path = create_path(iid, name);
        Cmd::write_pqt(df, &path).unwrap();

        log::info!("Analytic save {}", path.display());
    }
    fn load(iid: &IID, name: &str) -> Result<DataFrame, String> {
        let path = create_path(iid, name);

        if !Cmd::is_exist(&path) {
            let msg = format!("analyse not found: {}", path.display());
            return Err(msg);
        }

        load_file(path)
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

        let df = df!(
            "size" => sizes,
            "begin" => begin,
            "end" => end,
        )
        .unwrap();

        df
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

        let df = df!(
            "sz" => sizes,
            "begin" => begin,
            "end" => end,
        )
        .unwrap();

        df
    }
    fn cdf<T>(value: T, cdf_df: DataFrame) -> f64
    where
        T: NumericNative + Into<Expr>,
    {
        let cdf = cdf_df
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
            .unwrap();

        cdf
    }
    fn cdf_p<T>(value: T, cdf_df: DataFrame) -> f64
    where
        T: NumericNative + Into<Expr>,
    {
        let cdf = cdf_df
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
            .unwrap();

        cdf
    }
    fn size<T>(value: T, sizes: &DataFrame) -> Size
    where
        T: NumericNative + Into<Expr>,
    {
        let filtered = sizes
            .clone()
            .lazy()
            .filter(col("begin").lt_eq(value.clone()))
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
            return Size::GreatestBig;
        } else {
            return Size::GreatestSmall;
        }
    }
    fn sz<T>(value: T, sizes: &DataFrame) -> Sz
    where
        T: NumericNative + Into<Expr>,
    {
        let filtered = sizes
            .clone()
            .lazy()
            .filter(col("begin").lt_eq(value.clone()))
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
        if greatest {
            return Sz::XL;
        } else {
            return Sz::XS;
        }
    }
}

// private
fn create_path(iid: &IID, analyse_name: &str) -> PathBuf {
    let mut path = iid.path();

    path.push("ANALYSE");

    let analyse_name = format!("{}.pqt", analyse_name);
    for part in analyse_name.split(' ') {
        path.push(part);
    }

    path
}
#[cached]
fn load_file(path: PathBuf) -> Result<DataFrame, String> {
    let result = Cmd::read_pqt(&path);
    match result {
        Ok(df) => {
            log::info!("Analytic load {}", path.display());
            Ok(df)
        }
        Err(why) => Err(format!("{}", why)),
    }
}
