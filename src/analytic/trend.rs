/*****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::TimeZone;
use chrono::Utc;
use polars::df;
use polars::frame::DataFrame;
use polars::prelude::IntoLazy;
use polars::prelude::NamedFrom;
use polars::prelude::col;
use polars::prelude::lit;
use polars::series::Series;
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::Chart;
use crate::ChartFeatures;
use crate::IID;
use crate::Share;
use crate::Size;
use crate::Term;
use crate::TimeFrame;
use crate::Trend;
use crate::utils;

use super::Analytic;
pub struct TrendAnalytic {}
impl TrendAnalytic {
    pub fn abs_size(trend: &Trend) -> Option<Size> {
        let result = get_sizes_df(trend, &Feat::Abs);

        match result {
            Ok(sizes) => Some(Self::size(trend.abs_p(), &sizes)),
            Err(_) => None,
        }
    }
    pub fn len_size(trend: &Trend) -> Option<Size> {
        let result = get_sizes_df(trend, &Feat::Len);

        match result {
            Ok(sizes) => Some(Self::size(trend.len(), &sizes)),
            Err(_) => None,
        }
    }
    pub fn speed_size(trend: &Trend) -> Option<Size> {
        let result = get_sizes_df(trend, &Feat::Speed);

        match result {
            Ok(sizes) => Some(Self::size(trend.speed_p(), &sizes)),
            Err(_) => None,
        }
    }
    pub fn vol_size(trend: &Trend) -> Option<Size> {
        let result = get_sizes_df(trend, &Feat::Vol);

        match result {
            Ok(sizes) => Some(Self::size(trend.vol_total(), &sizes)),
            Err(_) => None,
        }
    }

    pub fn abs_cdf(trend: &Trend) -> Option<f64> {
        let result = get_cdf_df(trend, &Feat::Abs);

        match result {
            Ok(cdf_df) => Some(Self::cdf(trend.abs_p(), cdf_df)),
            Err(_) => None,
        }
    }
    pub fn len_cdf(trend: &Trend) -> Option<f64> {
        let result = get_cdf_df(trend, &Feat::Len);

        match result {
            Ok(cdf_df) => Some(Self::cdf(trend.len(), cdf_df)),
            Err(_) => None,
        }
    }
    pub fn speed_cdf(trend: &Trend) -> Option<f64> {
        let result = get_cdf_df(trend, &Feat::Speed);

        match result {
            Ok(cdf_df) => Some(Self::cdf(trend.speed_p(), cdf_df)),
            Err(_) => None,
        }
    }
    pub fn vol_cdf(trend: &Trend) -> Option<f64> {
        let result = get_cdf_df(trend, &Feat::Vol);

        match result {
            Ok(cdf_df) => Some(Self::cdf(trend.vol_total(), cdf_df)),
            Err(_) => None,
        }
    }

    pub fn posterior(trend: &Trend) -> Option<DataFrame> {
        // try get abs size for this trend
        if Self::abs_size(trend).is_none() {
            return None;
        }

        // all - all historical trends
        // obs - observation trends for current 'trend'
        // step - for this 'trend', depends on timeframe
        let all = get_trends_df(trend)
            .unwrap()
            .with_row_index("id".into(), None)
            .unwrap();
        let obs = get_obs(trend, &all);
        let step = get_step(trend);

        // eval posterior
        let mut df = get_posterior(all, obs, step);

        // Если тренд бычий, значит следующий медвежий.
        // abs по модулю посчитан, так что для определения цен
        // текущего медвежьего тренда, надо abs умножить на -1
        let k = if trend.is_bull() { -1.0 } else { 1.0 };
        let delta = df.column("abs").unwrap() * k;

        // eval concrete prices from delta and current trend end price
        let price = trend.end().price;
        let mut price_column = delta * price / 100.0 + price;
        price_column.rename("price".into());

        df.with_column(price_column).unwrap();
        Some(df)
    }
}
impl Analytic for TrendAnalytic {
    #[inline]
    fn name() -> &'static str {
        "trend"
    }
    fn analyse(iid: &IID, tf: &TimeFrame) -> Result<(), String> {
        log::info!(":: Analyse trend {} {}", iid.ticker(), tf);

        let mut chart = load_chart(iid, tf).unwrap();
        chart.features(ChartFeatures::Extremum, true);

        for term in Term::iter() {
            // get all Trend
            let trends = chart.all_trends(&term);
            if trends.is_empty() {
                continue;
            }

            // create trends dataframe
            let mut trends_df = create_trends_df(&trends);

            // analyse features
            for feat in Feat::iter() {
                analyse_feat(iid, &trends_df, tf, &term, &feat);
            }

            // set trend analyse (cdf, size, sz)
            set_analyse_feat(&trends, &mut trends_df);

            // save
            let name = analyse_name(tf, &term, None, None);
            TrendAnalytic::save(iid, &name, &mut trends_df);
        }

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

// analyse
fn analyse_name(
    tf: &TimeFrame,
    term: &Term,
    feat: Option<&Feat>,
    analyse: Option<&Analyse>,
) -> String {
    if feat.is_some() && analyse.is_some() {
        format!(
            "{} {} {} {} {}",
            TrendAnalytic::name(),
            tf,
            term,
            feat.unwrap().name(),
            analyse.unwrap().name(),
        )
    } else {
        format!("{} {} {} trend", TrendAnalytic::name(), tf, term,)
    }
}
fn load_chart(iid: &IID, tf: &TimeFrame) -> Result<Chart, String> {
    log::info!("   Load chart");

    let begin = Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap();
    let end = Utc::now();

    let chart = Chart::load(iid, tf, &begin, &end).unwrap();

    Ok(chart)
}
fn create_trends_df(trends: &Vec<Trend>) -> DataFrame {
    let term = trends[0].term();
    log::info!("   Create trends dataframe {}", term);

    // tmp vectors for create df
    let mut begin = Vec::new();
    let mut end = Vec::new();
    let mut begin_price = Vec::new();
    let mut end_price = Vec::new();
    let mut kind = Vec::new();
    let mut len = Vec::new();
    let mut abs = Vec::new();
    let mut speed = Vec::new();
    let mut vol_bear = Vec::new();
    let mut vol_bull = Vec::new();
    let mut vol_total = Vec::new();

    // collect values
    for trend in trends.iter() {
        begin.push(trend.begin().ts_nanos);
        end.push(trend.end().ts_nanos);
        begin_price.push(trend.begin().price);
        end_price.push(trend.end().price);
        kind.push(if trend.is_bull() { "Bull" } else { "Bear" });
        len.push(trend.len());
        abs.push(trend.abs_p());
        speed.push(trend.speed_p());
        vol_bear.push(trend.vol_bear());
        vol_bull.push(trend.vol_bull());
        vol_total.push(trend.vol_total());
    }

    // create df
    let df = df!(
            "begin" => begin,
            "end" => end,
            "begin_price" => begin_price,
            "end_price" => end_price,
            "kind" => kind,
            Feat::Abs.name() => abs,
            Feat::Len.name() => len,
            Feat::Speed.name() => speed,
            Feat::Vol.name() => vol_total,
            Feat::VolBear.name() => vol_bear,
            Feat::VolBull.name() => vol_bull,
    )
    .unwrap();

    df
}
fn analyse_feat(
    iid: &IID,
    trends: &DataFrame,
    tf: &TimeFrame,
    term: &Term,
    feat: &Feat,
) {
    let analyse = Analyse::CDF;
    let name = analyse_name(tf, term, Some(&feat), Some(&analyse));
    let mut cdf = TrendAnalytic::eval_cdf(
        trends.column(feat.name()).unwrap().as_materialized_series(),
    );
    TrendAnalytic::save(iid, &name, &mut cdf);

    let analyse = Analyse::Size;
    let mut sizes = TrendAnalytic::eval_size(&cdf);
    let name = analyse_name(tf, term, Some(&feat), Some(&analyse));
    TrendAnalytic::save(iid, &name, &mut sizes);

    let analyse = Analyse::Sz;
    let mut sizes = TrendAnalytic::eval_sz(&cdf);
    let name = analyse_name(tf, term, Some(&feat), Some(&analyse));
    TrendAnalytic::save(iid, &name, &mut sizes);
}
fn set_analyse_feat(trends: &Vec<Trend>, trends_df: &mut DataFrame) {
    // tmp vectors for columns
    let mut abs_cdf = Vec::new();
    let mut abs_size = Vec::new();
    let mut abs_sz = Vec::new();
    let mut len_cdf = Vec::new();
    let mut len_size = Vec::new();
    let mut len_sz = Vec::new();
    let mut speed_cdf = Vec::new();
    let mut speed_size = Vec::new();
    let mut speed_sz = Vec::new();
    let mut vol_cdf = Vec::new();
    let mut vol_size = Vec::new();
    let mut vol_sz = Vec::new();

    // collect values
    for trend in trends.iter() {
        let cdf = TrendAnalytic::abs_cdf(trend).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        abs_cdf.push(cdf);
        abs_size.push(size.name());
        abs_sz.push(sz.name());

        let cdf = TrendAnalytic::len_cdf(trend).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        len_cdf.push(cdf);
        len_size.push(size.name());
        len_sz.push(sz.name());

        let cdf = TrendAnalytic::speed_cdf(trend).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        speed_cdf.push(cdf);
        speed_size.push(size.name());
        speed_sz.push(sz.name());

        let cdf = TrendAnalytic::vol_cdf(trend).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        vol_cdf.push(cdf);
        vol_size.push(size.name());
        vol_sz.push(sz.name());
    }

    // Vec -> Series
    let abs_cdf = Series::new("abs_cdf".into(), abs_cdf);
    let abs_size = Series::new("abs_size".into(), abs_size);
    let abs_sz = Series::new("abs_sz".into(), abs_sz);

    let len_cdf = Series::new("len_cdf".into(), len_cdf);
    let len_size = Series::new("len_size".into(), len_size);
    let len_sz = Series::new("len_sz".into(), len_sz);

    let speed_cdf = Series::new("speed_cdf".into(), speed_cdf);
    let speed_size = Series::new("speed_size".into(), speed_size);
    let speed_sz = Series::new("speed_sz".into(), speed_sz);

    let vol_cdf = Series::new("vol_cdf".into(), vol_cdf);
    let vol_size = Series::new("vol_size".into(), vol_size);
    let vol_sz = Series::new("vol_sz".into(), vol_sz);

    // add columns
    trends_df.with_column(abs_cdf).unwrap();
    trends_df.with_column(abs_size).unwrap();
    trends_df.with_column(abs_sz).unwrap();

    trends_df.with_column(len_cdf).unwrap();
    trends_df.with_column(len_size).unwrap();
    trends_df.with_column(len_sz).unwrap();

    trends_df.with_column(speed_cdf).unwrap();
    trends_df.with_column(speed_size).unwrap();
    trends_df.with_column(speed_sz).unwrap();

    trends_df.with_column(vol_cdf).unwrap();
    trends_df.with_column(vol_size).unwrap();
    trends_df.with_column(vol_sz).unwrap();
}
fn get_trends_df(trend: &Trend) -> Result<DataFrame, String> {
    let iid = trend.chart().iid();
    let tf = trend.tf();
    let term = trend.term();
    let name = analyse_name(tf, term, None, None);

    TrendAnalytic::load(iid, &name)
}
fn get_cdf_df(trend: &Trend, feat: &Feat) -> Result<DataFrame, String> {
    // df:
    // ┌───────┬───────┬─────────────┬──────────┬─────────────┐
    // │ value ┆ count ┆ probability ┆ cdf      ┆ cdf_p       │
    // │ ---   ┆ ---   ┆ ---         ┆ ---      ┆ ---         │
    // │ f64   ┆ u32   ┆ f64         ┆ f64      ┆ f64         │
    // ╞═══════╪═══════╪═════════════╪══════════╪═════════════╡
    // │ 0.0   ┆ 545   ┆ 0.014626    ┆ 0.014626 ┆ 1.462616    │
    // │ …     ┆ …     ┆ …           ┆ …        ┆ …           │

    let iid = trend.chart().iid();
    let tf = trend.tf();
    let term = trend.term();
    let analyse = Analyse::CDF;
    let name = analyse_name(tf, term, Some(&feat), Some(&analyse));

    TrendAnalytic::load(iid, &name)
}
fn get_sizes_df(trend: &Trend, feat: &Feat) -> Result<DataFrame, String> {
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

    let iid = trend.chart().iid();
    let tf = trend.tf();
    let term = trend.term();
    let analyse = Analyse::Size;
    let name = analyse_name(tf, term, Some(&feat), Some(&analyse));

    TrendAnalytic::load(iid, &name)
}

// posterior
const MIN_SAMPLE: usize = 1000;
fn get_obs(trend: &Trend, all: &DataFrame) -> DataFrame {
    // stage 1 - abs size
    let value = TrendAnalytic::abs_size(trend).unwrap().name();
    let obs_1 = all
        .clone()
        .lazy()
        .filter(col("abs_size").eq(lit(value)))
        .collect()
        .unwrap();
    if obs_1.height() < MIN_SAMPLE {
        return obs_1;
    }

    // stage 2 - speed sz
    let value = TrendAnalytic::speed_size(trend).unwrap().sz().name();
    let obs_2 = obs_1
        .clone()
        .lazy()
        .filter(col("speed_sz").eq(lit(value)))
        .collect()
        .unwrap();
    if obs_2.height() < MIN_SAMPLE {
        return obs_1;
    }

    // stage 3 - vol sz
    let value = TrendAnalytic::vol_size(trend).unwrap().sz().name();
    let obs_3 = obs_2
        .clone()
        .lazy()
        .filter(col("vol_sz").eq(lit(value)))
        .collect()
        .unwrap();
    if obs_3.height() < MIN_SAMPLE {
        return obs_2;
    } else {
        return obs_3;
    }
}
fn get_step(trend: &Trend) -> f64 {
    match trend.tf() {
        TimeFrame::M1 => 0.01,
        // TimeFrame::M5 => 0.05,
        TimeFrame::M10 => 0.10,
        TimeFrame::H1 => 0.20,
        TimeFrame::Day => 0.25,
        TimeFrame::Week => 0.50,
        TimeFrame::Month => 1.00,
    }
}
fn get_posterior(all: DataFrame, obs: DataFrame, step: f64) -> DataFrame {
    // obs_id - observation trend id
    // h_id - hypothesis trend id
    let obs_id = obs.column("id").unwrap();
    let h_id = obs_id + 1;
    let h_id = h_id.as_materialized_series().clone();

    // tmp Vec for create df
    let mut abs = Vec::new();
    let mut probability = Vec::new();

    // x - trend abs
    // p - probability of this abs
    let mut x: f64 = 0.0;
    let mut p: f64 = 100.0;
    let mut combo = all;
    while p >= 1.0 {
        combo = combo
            .lazy()
            .filter(col("id").is_in(lit(h_id.clone())))
            .filter(col("abs").gt(lit(x)))
            .collect()
            .unwrap();

        p = combo.height() as f64 / obs.height() as f64 * 100.0;
        probability.push(p);
        abs.push(x);

        x = utils::round(x + step, 2);
    }

    df!(
        "abs" => abs,
        "p" => probability,
    )
    .unwrap()

    //     begin = 0
    //     end = trends["abs"].max()
    //
    //     delta = list()
    //     posterior = list()
    //     for x in np.arange(begin, end, step):
    //         combo = trends.filter(
    //             pl.col("index").is_in(h_index),
    //             pl.col("abs") > x,
    //         )
    //         p = len(combo) / len(obs_trends)
    //
    //         delta.append(x)
    //         posterior.append(p)
    //
    //         if p <= 0.01:
    //             break
    //
    //     result = pl.DataFrame(
    //         {
    //             "abs": delta,
    //             "p": posterior,
    //         }
    //     )
    //     result = result.with_columns(
    //         cumulative = pl.col("p") * 100,
    //     )
    //     return result
}

#[derive(Debug)]
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
    Abs,
    Len,
    Speed,
    Vol,
    VolBear,
    VolBull,
}
impl Feat {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Abs => "abs",
            Self::Len => "len",
            Self::Speed => "speed",
            Self::Vol => "vol",
            Self::VolBear => "vol_bear",
            Self::VolBull => "vol_bull",
        }
    }
}
