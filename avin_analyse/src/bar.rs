/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{TimeZone, Utc};
use polars::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use avin_core::{Bar, Chart, Iid, Share, TimeFrame};
use avin_utils::AvinError;

use crate::{Analyse, Size};

const NAME: &str = "bar";

#[derive(Debug, Clone, Copy, EnumIter)]
enum Metric {
    Cdf,
    Size,
    Sz,
}
impl Metric {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Cdf => "cdf",
            Self::Size => "size",
            Self::Sz => "sz",
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Feat {
    Body,
    Full,
    Lower,
    Upper,
    Value,
    Volume,
}
impl Feat {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Full => "full",
            Self::Body => "body",
            Self::Lower => "lower",
            Self::Upper => "upper",
            Self::Volume => "volume",
            Self::Value => "value",
        }
    }
}

impl Analyse for Bar {
    fn analyse(iid: &Iid, tf: TimeFrame) -> Result<(), AvinError> {
        log::info!(":: Analyse {} {} {}", NAME, iid.ticker(), tf);

        // load chart
        let chart = load_chart(iid, tf).unwrap();

        // create volumes dataframe
        let mut vol_df = create_df(&chart);

        // analyse features
        for feat in Feat::iter() {
            analyse_feat(iid, &vol_df, tf, &feat);
        }

        // set trend analyse (cdf, size, sz)
        set_metrics(&chart, &mut vol_df);

        // save volumes
        let name = analyse_name(chart.tf(), None, None);
        Bar::save(chart.iid(), &name, &mut vol_df);

        Ok(())
    }
    fn analyse_all() -> Result<(), AvinError> {
        let shares = Share::all();
        let timeframes = TimeFrame::all();

        for share in shares.iter() {
            for tf in timeframes.iter() {
                Self::analyse(share.iid(), *tf).unwrap();
            }
        }

        Ok(())
    }
}

// public interface for Chart
pub trait BarAnalytic {
    fn init(&mut self);

    fn bar_body_cdf(&self, bar: &Bar) -> Option<f64>;
    fn bar_full_cdf(&self, bar: &Bar) -> Option<f64>;
    fn bar_lower_cdf(&self, bar: &Bar) -> Option<f64>;
    fn bar_upper_cdf(&self, bar: &Bar) -> Option<f64>;
    fn bar_vol_cdf(&self, bar: &Bar) -> Option<f64>;
    fn bar_val_cdf(&self, bar: &Bar) -> Option<f64>;

    fn bar_body_size(&self, bar: &Bar) -> Option<Size>;
    fn bar_full_size(&self, bar: &Bar) -> Option<Size>;
    fn bar_lower_size(&self, bar: &Bar) -> Option<Size>;
    fn bar_upper_size(&self, bar: &Bar) -> Option<Size>;
    fn bar_vol_size(&self, bar: &Bar) -> Option<Size>;
    fn bar_val_size(&self, bar: &Bar) -> Option<Size>;
}
impl BarAnalytic for Chart {
    fn init(&mut self) {}
    fn bar_body_cdf(&self, bar: &Bar) -> Option<f64> {
        match get_cdf_df(self, &Feat::Body) {
            Ok(cdf_df) => Some(Bar::cdf(bar.body().abs_p(), cdf_df)),
            Err(_) => None,
        }
    }
    fn bar_full_cdf(&self, bar: &Bar) -> Option<f64> {
        match get_cdf_df(self, &Feat::Full) {
            Ok(cdf_df) => Some(Bar::cdf(bar.full().abs_p(), cdf_df)),
            Err(_) => None,
        }
    }
    fn bar_lower_cdf(&self, bar: &Bar) -> Option<f64> {
        match get_cdf_df(self, &Feat::Lower) {
            Ok(cdf_df) => Some(Bar::cdf(bar.lower().abs_p(), cdf_df)),
            Err(_) => None,
        }
    }
    fn bar_upper_cdf(&self, bar: &Bar) -> Option<f64> {
        match get_cdf_df(self, &Feat::Upper) {
            Ok(cdf_df) => Some(Bar::cdf(bar.upper().abs_p(), cdf_df)),
            Err(_) => None,
        }
    }
    fn bar_vol_cdf(&self, bar: &Bar) -> Option<f64> {
        match get_cdf_df(self, &Feat::Volume) {
            Ok(cdf_df) => Some(Bar::cdf(bar.v, cdf_df)),
            Err(_) => None,
        }
    }
    fn bar_val_cdf(&self, bar: &Bar) -> Option<f64> {
        match get_cdf_df(self, &Feat::Value) {
            Ok(cdf_df) => Some(Bar::cdf(bar.val.unwrap(), cdf_df)),
            Err(_) => None,
        }
    }

    fn bar_body_size(&self, bar: &Bar) -> Option<Size> {
        match get_sizes_df(self, &Feat::Body) {
            Ok(sizes) => Some(Bar::size(bar.body().abs_p(), &sizes)),
            Err(_) => None,
        }
    }
    fn bar_full_size(&self, bar: &Bar) -> Option<Size> {
        match get_sizes_df(self, &Feat::Full) {
            Ok(sizes) => Some(Bar::size(bar.full().abs_p(), &sizes)),
            Err(_) => None,
        }
    }
    fn bar_lower_size(&self, bar: &Bar) -> Option<Size> {
        match get_sizes_df(self, &Feat::Lower) {
            Ok(sizes) => Some(Bar::size(bar.lower().abs_p(), &sizes)),
            Err(_) => None,
        }
    }
    fn bar_upper_size(&self, bar: &Bar) -> Option<Size> {
        match get_sizes_df(self, &Feat::Upper) {
            Ok(sizes) => Some(Bar::size(bar.upper().abs_p(), &sizes)),
            Err(_) => None,
        }
    }
    fn bar_vol_size(&self, bar: &Bar) -> Option<Size> {
        match get_sizes_df(self, &Feat::Volume) {
            Ok(sizes) => Some(Bar::size(bar.v, &sizes)),
            Err(_) => None,
        }
    }
    fn bar_val_size(&self, bar: &Bar) -> Option<Size> {
        match get_sizes_df(self, &Feat::Value) {
            Ok(sizes) => Some(Bar::size(bar.val.unwrap(), &sizes)),
            Err(_) => None,
        }
    }
}

// analyse
fn analyse_name(
    tf: TimeFrame,
    feat: Option<&Feat>,
    metric: Option<&Metric>,
) -> String {
    if feat.is_some() && metric.is_some() {
        format!(
            "{} {} {} {}",
            NAME,
            tf,
            feat.unwrap().name(),
            metric.unwrap().name(),
        )
    } else {
        format!("{NAME} {tf} {NAME}")
    }
}
fn load_chart(iid: &Iid, tf: TimeFrame) -> Result<Chart, String> {
    log::info!("Load chart {tf}");

    let begin = Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap();
    let end = Utc::now();

    let chart = Chart::load(iid, tf, &begin, &end).unwrap();

    Ok(chart)
}
fn create_df(chart: &Chart) -> DataFrame {
    log::info!("Create volumes dataframe");

    // tmp vector
    let mut ts_nanos = Vec::new();
    let mut full = Vec::new();
    let mut body = Vec::new();
    let mut upper = Vec::new();
    let mut lower = Vec::new();
    let mut volumes = Vec::new();
    let mut values = Vec::new();

    // collect values
    for bar in chart.bars().iter() {
        ts_nanos.push(bar.ts_nanos);
        full.push(bar.full().abs_p());
        body.push(bar.body().abs_p());
        upper.push(bar.upper().abs_p());
        lower.push(bar.lower().abs_p());
        volumes.push(bar.v);
        values.push(bar.val);
    }

    // create df
    df!(
            "ts_nanos" => ts_nanos,
            Feat::Body.name() => body,
            Feat::Full.name() => full,
            Feat::Lower.name() => lower,
            Feat::Upper.name() => upper,
            Feat::Value.name() => values,
            Feat::Volume.name() => volumes,
    )
    .unwrap()
}
fn analyse_feat(iid: &Iid, vol_df: &DataFrame, tf: TimeFrame, feat: &Feat) {
    log::info!("Analyse feat {}", feat.name());

    let metric = Metric::Cdf;
    let name = analyse_name(tf, Some(feat), Some(&metric));
    let mut cdf = Bar::eval_cdf(
        vol_df.column(feat.name()).unwrap().as_materialized_series(),
    );
    Bar::save(iid, &name, &mut cdf);

    let metric = Metric::Size;
    let mut sizes = Bar::eval_size(&cdf);
    let name = analyse_name(tf, Some(feat), Some(&metric));
    Bar::save(iid, &name, &mut sizes);

    let metric = Metric::Sz;
    let mut szs = Bar::eval_sz(&cdf);
    let name = analyse_name(tf, Some(feat), Some(&metric));
    Bar::save(iid, &name, &mut szs);
}
fn get_cdf_df(chart: &Chart, feat: &Feat) -> Result<DataFrame, AvinError> {
    // df:
    // ┌───────┬───────┬───────┐
    // │ size  ┆ begin ┆ end   │
    // │ ---   ┆ ---   ┆ ---   │
    // │ str   ┆ f64   ┆ f64   │
    // ╞═══════╪═══════╪═══════╡
    // │ XS    ┆ 0.0   ┆ 0.39  │
    // │ S     ┆ 0.39  ┆ 0.83  │
    // │ M     ┆ 0.83  ┆ 2.05  │
    // │ L     ┆ 2.05  ┆ 3.9   │
    // │ XL    ┆ 3.9   ┆ 52.76 │
    // └───────┴───────┴───────┘

    let iid = chart.iid();
    let tf = chart.tf();
    let metric = Metric::Cdf;
    let name = analyse_name(tf, Some(feat), Some(&metric));

    Bar::load(iid, &name)
}
fn get_sizes_df(chart: &Chart, feat: &Feat) -> Result<DataFrame, AvinError> {
    // df:
    // ┌───────┬───────┬───────┐
    // │ size  ┆ begin ┆ end   │
    // │ ---   ┆ ---   ┆ ---   │
    // │ str   ┆ f64   ┆ f64   │
    // ╞═══════╪═══════╪═══════╡
    // │ XS    ┆ 0.0   ┆ 0.39  │
    // │ S     ┆ 0.39  ┆ 0.83  │
    // │ M     ┆ 0.83  ┆ 2.05  │
    // │ L     ┆ 2.05  ┆ 3.9   │
    // │ XL    ┆ 3.9   ┆ 52.76 │
    // └───────┴───────┴───────┘

    let iid = chart.iid();
    let tf = chart.tf();
    let metric = Metric::Size;
    let name = analyse_name(tf, Some(feat), Some(&metric));

    Bar::load(iid, &name)
}
fn set_metrics(chart: &Chart, vol_df: &mut DataFrame) {
    log::info!("Set analyse");

    // tmp vectors for columns
    let mut body_cdf = Vec::new();
    let mut body_size = Vec::new();
    let mut body_sz = Vec::new();
    let mut full_cdf = Vec::new();
    let mut full_size = Vec::new();
    let mut full_sz = Vec::new();
    let mut lower_cdf = Vec::new();
    let mut lower_size = Vec::new();
    let mut lower_sz = Vec::new();
    let mut upper_cdf = Vec::new();
    let mut upper_size = Vec::new();
    let mut upper_sz = Vec::new();

    let mut vol_cdf = Vec::new();
    let mut vol_size = Vec::new();
    let mut vol_sz = Vec::new();
    let mut val_cdf = Vec::new();
    let mut val_size = Vec::new();
    let mut val_sz = Vec::new();

    // collect values
    for bar in chart.bars().iter() {
        let cdf = chart.bar_body_cdf(bar).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        body_cdf.push(cdf);
        body_size.push(size.name());
        body_sz.push(sz.name());

        let cdf = chart.bar_full_cdf(bar).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        full_cdf.push(cdf);
        full_size.push(size.name());
        full_sz.push(sz.name());

        let cdf = chart.bar_lower_cdf(bar).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        lower_cdf.push(cdf);
        lower_size.push(size.name());
        lower_sz.push(sz.name());

        let cdf = chart.bar_upper_cdf(bar).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        upper_cdf.push(cdf);
        upper_size.push(size.name());
        upper_sz.push(sz.name());

        let cdf = chart.bar_vol_cdf(bar).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        vol_cdf.push(cdf);
        vol_size.push(size.name());
        vol_sz.push(sz.name());

        let cdf = chart.bar_val_cdf(bar).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        val_cdf.push(cdf);
        val_size.push(size.name());
        val_sz.push(sz.name());
    }

    // Vec -> Series
    let body_cdf = Series::new("body_cdf".into(), body_cdf);
    let body_size = Series::new("body_size".into(), body_size);
    let body_sz = Series::new("body_sz".into(), body_sz);

    let full_cdf = Series::new("full_cdf".into(), full_cdf);
    let full_size = Series::new("full_size".into(), full_size);
    let full_sz = Series::new("full_sz".into(), full_sz);

    let lower_cdf = Series::new("lower_cdf".into(), lower_cdf);
    let lower_size = Series::new("lower_size".into(), lower_size);
    let lower_sz = Series::new("lower_sz".into(), lower_sz);

    let upper_cdf = Series::new("upper_cdf".into(), upper_cdf);
    let upper_size = Series::new("upper_size".into(), upper_size);
    let upper_sz = Series::new("upper_sz".into(), upper_sz);

    let vol_cdf = Series::new("vol_cdf".into(), vol_cdf);
    let vol_size = Series::new("vol_size".into(), vol_size);
    let vol_sz = Series::new("vol_sz".into(), vol_sz);

    let val_cdf = Series::new("val_cdf".into(), val_cdf);
    let val_size = Series::new("val_size".into(), val_size);
    let val_sz = Series::new("val_sz".into(), val_sz);

    // add columns
    vol_df.with_column(body_cdf).unwrap();
    vol_df.with_column(body_size).unwrap();
    vol_df.with_column(body_sz).unwrap();

    vol_df.with_column(full_cdf).unwrap();
    vol_df.with_column(full_size).unwrap();
    vol_df.with_column(full_sz).unwrap();

    vol_df.with_column(lower_cdf).unwrap();
    vol_df.with_column(lower_size).unwrap();
    vol_df.with_column(lower_sz).unwrap();

    vol_df.with_column(upper_cdf).unwrap();
    vol_df.with_column(upper_size).unwrap();
    vol_df.with_column(upper_sz).unwrap();

    vol_df.with_column(vol_cdf).unwrap();
    vol_df.with_column(vol_size).unwrap();
    vol_df.with_column(vol_sz).unwrap();

    vol_df.with_column(val_cdf).unwrap();
    vol_df.with_column(val_size).unwrap();
    vol_df.with_column(val_sz).unwrap();
}
