/*****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::TimeZone;
use chrono::Utc;
use polars::prelude::*;
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::Bar;
use crate::Chart;
use crate::IID;
use crate::Share;
use crate::Size;
use crate::TimeFrame;

use super::Analytic;
pub struct VolumeAnalytic {}
impl VolumeAnalytic {
    pub fn vol_cdf(chart: &Chart, bar: &Bar) -> Option<f64> {
        match get_cdf_df(chart, &Feat::Volume) {
            Ok(cdf_df) => Some(Self::cdf(bar.v, cdf_df)),
            Err(_) => None,
        }
    }
    pub fn val_cdf(chart: &Chart, bar: &Bar) -> Option<f64> {
        match get_cdf_df(chart, &Feat::Value) {
            Ok(cdf_df) => Some(Self::cdf(bar.val.unwrap(), cdf_df)),
            Err(_) => None,
        }
    }
    pub fn vol_size(chart: &Chart, bar: &Bar) -> Option<Size> {
        match get_sizes_df(chart, &Feat::Volume) {
            Ok(sizes) => Some(Self::size(bar.v, &sizes)),
            Err(_) => None,
        }
    }
    pub fn val_size(chart: &Chart, bar: &Bar) -> Option<Size> {
        match get_sizes_df(chart, &Feat::Value) {
            Ok(sizes) => Some(Self::size(bar.val.unwrap(), &sizes)),
            Err(_) => None,
        }
    }
}
impl Analytic for VolumeAnalytic {
    #[inline]
    fn name() -> &'static str {
        "volume"
    }
    fn analyse(iid: &IID, tf: &TimeFrame) -> Result<(), String> {
        log::info!(":: Analyse {} {} {}", Self::name(), iid.ticker(), tf);

        // load chart
        let chart = load_chart(iid, tf).unwrap();

        // create volumes dataframe
        let mut vol_df = create_df(&chart);

        // analyse features
        for feat in Feat::iter() {
            analyse_feat(iid, &vol_df, tf, &feat);
        }

        // set trend analyse (cdf, size, sz)
        set_analyse_feat(&chart, &mut vol_df);

        Ok(())
    }
    fn analyse_all() -> Result<(), String> {
        let shares = Share::all();
        let timeframes = TimeFrame::all();

        for share in shares.iter() {
            for tf in timeframes.iter() {
                Self::analyse(share.iid(), tf).unwrap();
            }
        }

        Ok(())
    }
}

fn analyse_name(
    tf: &TimeFrame,
    feat: Option<&Feat>,
    analyse: Option<&Analyse>,
) -> String {
    if feat.is_some() && analyse.is_some() {
        format!(
            "{} {} {} {}",
            VolumeAnalytic::name(),
            tf,
            feat.unwrap().name(),
            analyse.unwrap().name(),
        )
    } else {
        format!("{} {} volumes", VolumeAnalytic::name(), tf)
    }
}
fn load_chart(iid: &IID, tf: &TimeFrame) -> Result<Chart, String> {
    log::info!("Load chart {}", tf);

    let begin = Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap();
    let end = Utc::now();

    let chart = Chart::load(iid, tf, &begin, &end).unwrap();

    Ok(chart)
}
fn create_df(chart: &Chart) -> DataFrame {
    log::info!("Create volumes dataframe");

    // tmp vector
    let mut ts_nanos = Vec::new();
    let mut volumes = Vec::new();
    let mut values = Vec::new();

    // collect values
    for bar in chart.bars().iter() {
        ts_nanos.push(bar.ts_nanos);
        volumes.push(bar.v);
        values.push(bar.val);
    }

    // create df
    let df = df!(
            "ts_nanos" => ts_nanos,
            Feat::Volume.name() => volumes,
            Feat::Value.name() => values,
    )
    .unwrap();

    df
}
fn analyse_feat(iid: &IID, vol_df: &DataFrame, tf: &TimeFrame, feat: &Feat) {
    log::info!("Analyse feat {}", feat.name());

    let analyse = Analyse::CDF;
    let name = analyse_name(tf, Some(feat), Some(&analyse));
    let mut cdf = VolumeAnalytic::eval_cdf(
        vol_df.column(feat.name()).unwrap().as_materialized_series(),
    );
    VolumeAnalytic::save(iid, &name, &mut cdf);

    // analyse size
    let analyse = Analyse::Size;
    let mut sizes = VolumeAnalytic::eval_size(&cdf);
    let name = analyse_name(tf, Some(feat), Some(&analyse));
    VolumeAnalytic::save(iid, &name, &mut sizes);

    let analyse = Analyse::Sz;
    let mut szs = VolumeAnalytic::eval_sz(&cdf);
    let name = analyse_name(tf, Some(feat), Some(&analyse));
    VolumeAnalytic::save(iid, &name, &mut szs);
}
fn set_analyse_feat(chart: &Chart, mut vol_df: &mut DataFrame) {
    log::info!("Set analyse");

    // tmp vectors for columns
    let mut vol_cdf = Vec::new();
    let mut vol_size = Vec::new();
    let mut vol_sz = Vec::new();
    let mut val_cdf = Vec::new();
    let mut val_size = Vec::new();
    let mut val_sz = Vec::new();

    // collect values
    for bar in chart.bars().iter() {
        let cdf = VolumeAnalytic::vol_cdf(chart, bar).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        vol_cdf.push(cdf);
        vol_size.push(size.name());
        vol_sz.push(sz.name());

        let cdf = VolumeAnalytic::val_cdf(chart, bar).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        val_cdf.push(cdf);
        val_size.push(size.name());
        val_sz.push(sz.name());
    }

    // Vec -> Series
    let vol_cdf = Series::new("vol_cdf".into(), vol_cdf);
    let vol_size = Series::new("vol_size".into(), vol_size);
    let vol_sz = Series::new("vol_sz".into(), vol_sz);

    let val_cdf = Series::new("val_cdf".into(), val_cdf);
    let val_size = Series::new("val_size".into(), val_size);
    let val_sz = Series::new("val_sz".into(), val_sz);

    // add columns
    vol_df.with_column(vol_cdf).unwrap();
    vol_df.with_column(vol_size).unwrap();
    vol_df.with_column(vol_sz).unwrap();

    vol_df.with_column(val_cdf).unwrap();
    vol_df.with_column(val_size).unwrap();
    vol_df.with_column(val_sz).unwrap();

    // save volumes
    let name = analyse_name(chart.tf(), None, None);
    VolumeAnalytic::save(chart.iid(), &name, &mut vol_df);
}
fn get_cdf_df(chart: &Chart, feat: &Feat) -> Result<DataFrame, String> {
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
    let analyse = Analyse::CDF;
    let name = analyse_name(tf, Some(&feat), Some(&analyse));

    VolumeAnalytic::load(iid, &name)
}
fn get_sizes_df(chart: &Chart, feat: &Feat) -> Result<DataFrame, String> {
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
    let analyse = Analyse::Size;
    let name = analyse_name(tf, Some(&feat), Some(&analyse));

    VolumeAnalytic::load(iid, &name)
}

#[derive(Debug, EnumIter)]
enum Analyse {
    CDF,
    Size,
    Sz,
}
impl Analyse {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::CDF => "cdf",
            Self::Size => "size",
            Self::Sz => "sz",
        }
    }
}

#[derive(Debug, EnumIter)]
enum Feat {
    Volume,
    Value,
}
impl Feat {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Volume => "volume",
            Self::Value => "value",
        }
    }
}
