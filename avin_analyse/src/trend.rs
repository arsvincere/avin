/*****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use cached::SizedCache;
use cached::proc_macro::cached;
use chrono::{TimeZone, Utc};
use polars::prelude::{DataFrame, IntoLazy, NamedFrom, Series, col, df, lit};
use strum::{EnumIter, IntoEnumIterator};

use avin_core::{
    Chart, ExtremumIndicator, Iid, Share, Term, TimeFrame, Trend,
};
use avin_utils::{self as utils, AvinError};

use crate::{Analyse, Size};

const NAME: &str = "trend";
const MIN_SAMPLE: usize = 1000;
const MIN_P: f64 = 10.0;

#[derive(Debug, Clone, Copy)]
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
    Abs,
    Len,
    Speed,
    Vol,
}
impl Feat {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Abs => "abs",
            Self::Len => "len",
            Self::Speed => "speed",
            Self::Vol => "vol",
        }
    }
}

impl Analyse for Trend {
    fn analyse(iid: &Iid, tf: &TimeFrame) -> Result<(), AvinError> {
        log::info!(":: Analyse {} {} {}", NAME, iid.ticker(), tf);

        let mut chart = load_chart(iid, tf).unwrap();
        ExtremumIndicator::init(&mut chart);

        for term in Term::iter() {
            // get all Trend
            let trends = chart.all_trend(term);
            if trends.is_empty() {
                continue;
            }

            // create trends dataframe
            let mut trends_df = create_trends_df(trends);

            // analyse features
            for feat in Feat::iter() {
                analyse_feat(iid, &trends_df, tf, term, feat);
            }

            // set trend metrics (cdf, size, sz)
            set_metrics(&chart, trends, &mut trends_df);

            // save
            let name = analyse_name(tf, term, None, None);
            Trend::save(iid, &name, &mut trends_df);
        }

        Ok(())
    }
    fn analyse_all() -> Result<(), AvinError> {
        let shares = Share::all();
        let timeframes = TimeFrame::all();

        for share in shares.iter() {
            for tf in timeframes.iter() {
                Trend::delete(share.iid(), NAME).unwrap();
                Self::analyse(share.iid(), tf).unwrap();
            }
        }

        Ok(())
    }
}

// public interface for Chart
pub trait TrendAnalytic: ExtremumIndicator {
    fn init(&mut self);

    fn trend_abs_size(&self, trend: &Trend) -> Option<Size>;
    fn trend_len_size(&self, trend: &Trend) -> Option<Size>;
    fn trend_speed_size(&self, trend: &Trend) -> Option<Size>;
    fn trend_vol_size(&self, trend: &Trend) -> Option<Size>;

    fn trend_abs_cdf(&self, trend: &Trend) -> Option<f64>;
    fn trend_len_cdf(&self, trend: &Trend) -> Option<f64>;
    fn trend_speed_cdf(&self, trend: &Trend) -> Option<f64>;
    fn trend_vol_cdf(&self, trend: &Trend) -> Option<f64>;

    fn trend_posterior_now(&self, term: Term) -> Option<DataFrame>;
    fn trend_posterior_last(&self, term: Term) -> Option<DataFrame>;
}
impl TrendAnalytic for Chart {
    fn init(&mut self) {}
    fn trend_abs_size(&self, trend: &Trend) -> Option<Size> {
        let result = get_sizes_df(self, trend, Feat::Abs);

        match result {
            Ok(sizes) => Some(Trend::size(trend.abs_p(), &sizes)),
            Err(_) => None,
        }
    }
    fn trend_len_size(&self, trend: &Trend) -> Option<Size> {
        let result = get_sizes_df(self, trend, Feat::Len);

        match result {
            Ok(sizes) => Some(Trend::size(trend.len(), &sizes)),
            Err(_) => None,
        }
    }
    fn trend_speed_size(&self, trend: &Trend) -> Option<Size> {
        let result = get_sizes_df(self, trend, Feat::Speed);

        match result {
            Ok(sizes) => Some(Trend::size(trend.speed_p(), &sizes)),
            Err(_) => None,
        }
    }
    fn trend_vol_size(&self, trend: &Trend) -> Option<Size> {
        let result = get_sizes_df(self, trend, Feat::Vol);

        match result {
            Ok(sizes) => Some(Trend::size(trend.vol(), &sizes)),
            Err(_) => None,
        }
    }

    fn trend_abs_cdf(&self, trend: &Trend) -> Option<f64> {
        let result = get_cdf_df(self, trend, Feat::Abs);

        match result {
            Ok(cdf_df) => Some(Trend::cdf(trend.abs_p(), cdf_df)),
            Err(_) => None,
        }
    }
    fn trend_len_cdf(&self, trend: &Trend) -> Option<f64> {
        let result = get_cdf_df(self, trend, Feat::Len);

        match result {
            Ok(cdf_df) => Some(Trend::cdf(trend.len(), cdf_df)),
            Err(_) => None,
        }
    }
    fn trend_speed_cdf(&self, trend: &Trend) -> Option<f64> {
        let result = get_cdf_df(self, trend, Feat::Speed);

        match result {
            Ok(cdf_df) => Some(Trend::cdf(trend.speed_p(), cdf_df)),
            Err(_) => None,
        }
    }
    fn trend_vol_cdf(&self, trend: &Trend) -> Option<f64> {
        let result = get_cdf_df(self, trend, Feat::Vol);

        match result {
            Ok(cdf_df) => Some(Trend::cdf(trend.vol(), cdf_df)),
            Err(_) => None,
        }
    }

    fn trend_posterior_now(&self, term: Term) -> Option<DataFrame> {
        // ┌──────┬───────────┬────────────┐
        // │ abs  ┆ p         ┆ price      │
        // │ ---  ┆ ---       ┆ ---        │
        // │ f64  ┆ f64       ┆ f64        │
        // ╞══════╪═══════════╪════════════╡
        // │ 0.0  ┆ 99.87545  ┆ 140.75     │
        // │ 0.01 ┆ 97.467479 ┆ 140.764075 │
        // │ 0.02 ┆ 92.928314 ┆ 140.77815  │
        // │ 0.03 ┆ 71.436479 ┆ 140.792225 │
        // │ 0.04 ┆ 62.413507 ┆ 140.8063   │
        // │ …    ┆ …         ┆ …          │
        // │ 0.24 ┆ 1.632992  ┆ 141.0878   │
        // │ 0.25 ┆ 1.480764  ┆ 141.101875 │
        // │ 0.26 ┆ 1.328536  ┆ 141.11595  │
        // │ 0.27 ┆ 1.079435  ┆ 141.130025 │
        // │ 0.28 ┆ 0.830335  ┆ 141.1441   │
        // └──────┴───────────┴────────────┘

        // try get abs size for this trend, if None -> None
        let trend = self.trend(term, 0)?;
        self.trend_abs_size(trend)?;

        cached_posterior_0(self, trend)
    }
    fn trend_posterior_last(&self, term: Term) -> Option<DataFrame> {
        // ┌──────┬───────────┬────────────┐
        // │ abs  ┆ p         ┆ price      │
        // │ ---  ┆ ---       ┆ ---        │
        // │ f64  ┆ f64       ┆ f64        │
        // ╞══════╪═══════════╪════════════╡
        // │ 0.0  ┆ 99.87545  ┆ 140.75     │
        // │ 0.01 ┆ 97.467479 ┆ 140.764075 │
        // │ 0.02 ┆ 92.928314 ┆ 140.77815  │
        // │ 0.03 ┆ 71.436479 ┆ 140.792225 │
        // │ 0.04 ┆ 62.413507 ┆ 140.8063   │
        // │ …    ┆ …         ┆ …          │
        // │ 0.24 ┆ 1.632992  ┆ 141.0878   │
        // │ 0.25 ┆ 1.480764  ┆ 141.101875 │
        // │ 0.26 ┆ 1.328536  ┆ 141.11595  │
        // │ 0.27 ┆ 1.079435  ┆ 141.130025 │
        // │ 0.28 ┆ 0.830335  ┆ 141.1441   │
        // └──────┴───────────┴────────────┘

        // try get abs size for this trend
        let trend = self.trend(term, 1)?;
        self.trend_abs_size(trend)?;

        cached_posterior_1(self, trend)
    }
}

// analyse
fn analyse_name(
    tf: &TimeFrame,
    term: Term,
    feat: Option<Feat>,
    metric: Option<Metric>,
) -> String {
    if feat.is_some() && metric.is_some() {
        format!(
            "{} {} {} {} {}",
            NAME,
            tf,
            term,
            feat.unwrap().name(),
            metric.unwrap().name(),
        )
    } else {
        format!("{NAME} {tf} {term} {NAME}")
    }
}
fn load_chart(iid: &Iid, tf: &TimeFrame) -> Result<Chart, AvinError> {
    log::info!("Load chart {tf}");

    let begin = Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap();
    let end = Utc::now();

    let chart = Chart::load(iid, tf, &begin, &end).unwrap();

    Ok(chart)
}
fn create_trends_df(trends: &[Trend]) -> DataFrame {
    let term = trends[0].term();
    log::info!("Create trends dataframe {term}");

    // tmp vectors for create df
    let mut begin = Vec::new();
    let mut end = Vec::new();
    let mut begin_price = Vec::new();
    let mut end_price = Vec::new();
    let mut kind = Vec::new();
    let mut len = Vec::new();
    let mut abs = Vec::new();
    let mut speed = Vec::new();
    let mut vol = Vec::new();

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
        vol.push(trend.vol());
    }

    df!(
        "begin" => begin,
        "end" => end,
        "begin_price" => begin_price,
        "end_price" => end_price,
        "kind" => kind,
        Feat::Abs.name() => abs,
        Feat::Len.name() => len,
        Feat::Speed.name() => speed,
        Feat::Vol.name() => vol,
    )
    .unwrap()
}
fn analyse_feat(
    iid: &Iid,
    trends: &DataFrame,
    tf: &TimeFrame,
    term: Term,
    feat: Feat,
) {
    log::info!("Analyse feat {}", feat.name());

    let metric = Metric::Cdf;
    let name = analyse_name(tf, term, Some(feat), Some(metric));
    let mut cdf = Trend::eval_cdf(
        trends.column(feat.name()).unwrap().as_materialized_series(),
    );
    Trend::save(iid, &name, &mut cdf);

    let metric = Metric::Size;
    let mut sizes = Trend::eval_size(&cdf);
    let name = analyse_name(tf, term, Some(feat), Some(metric));
    Trend::save(iid, &name, &mut sizes);

    let metric = Metric::Sz;
    let mut sizes = Trend::eval_sz(&cdf);
    let name = analyse_name(tf, term, Some(feat), Some(metric));
    Trend::save(iid, &name, &mut sizes);
}
fn get_trends_df(chart: &Chart, term: Term) -> Result<DataFrame, AvinError> {
    let iid = chart.iid();
    let tf = chart.tf();
    let name = analyse_name(tf, term, None, None);
    let df = Trend::load(iid, &name)?;

    Ok(df.with_row_index("id".into(), None).unwrap())
}
fn get_cdf_df(
    chart: &Chart,
    trend: &Trend,
    feat: Feat,
) -> Result<DataFrame, AvinError> {
    // df:
    // ┌───────┬───────┬─────────────┬──────────┬─────────────┐
    // │ value ┆ count ┆ probability ┆ cdf      ┆ cdf_p       │
    // │ ---   ┆ ---   ┆ ---         ┆ ---      ┆ ---         │
    // │ f64   ┆ u32   ┆ f64         ┆ f64      ┆ f64         │
    // ╞═══════╪═══════╪═════════════╪══════════╪═════════════╡
    // │ 0.0   ┆ 545   ┆ 0.014626    ┆ 0.014626 ┆ 1.462616    │
    // │ …     ┆ …     ┆ …           ┆ …        ┆ …           │

    let iid = chart.iid();
    let tf = chart.tf();
    let term = trend.term();
    let metric = Metric::Cdf;
    let name = analyse_name(tf, term, Some(feat), Some(metric));

    Trend::load(iid, &name)
}
fn get_sizes_df(
    chart: &Chart,
    trend: &Trend,
    feat: Feat,
) -> Result<DataFrame, AvinError> {
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
    let term = trend.term();
    let metric = Metric::Size;
    let name = analyse_name(tf, term, Some(feat), Some(metric));

    Trend::load(iid, &name)
}
fn set_metrics(chart: &Chart, trends: &[Trend], trends_df: &mut DataFrame) {
    log::info!("Set analyse");

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
        let cdf = chart.trend_abs_cdf(trend).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        abs_cdf.push(cdf);
        abs_size.push(size.name());
        abs_sz.push(sz.name());

        let cdf = chart.trend_len_cdf(trend).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        len_cdf.push(cdf);
        len_size.push(size.name());
        len_sz.push(sz.name());

        let cdf = chart.trend_speed_cdf(trend).unwrap();
        let size = Size::from_cdf(cdf);
        let sz = size.sz();
        speed_cdf.push(cdf);
        speed_size.push(size.name());
        speed_sz.push(sz.name());

        let cdf = chart.trend_vol_cdf(trend).unwrap();
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

// posterior
// сделано отдельной функцией чтобы отдельно кешировался 0 и 1
#[cached(
    ty = "SizedCache<i64, Option<DataFrame>>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ trend.begin().ts_nanos }"#
)]
fn cached_posterior_0(chart: &Chart, trend: &Trend) -> Option<DataFrame> {
    // n - current trend
    // all - all historical trends
    // obs - observation trends for current 'trend'
    // step - for this 'trend', depends on timeframe
    let all = get_trends_df(chart, trend.term()).unwrap();
    let obs = get_obs(chart, trend, &all);
    let step = get_step(chart.tf());

    // eval posterior
    let mut df = calc_posgerior(all, obs, step);

    // Если тренд бычий, значит следующий медвежий.
    // abs по модулю посчитан, так что для определения цен
    // текущего медвежьего тренда, надо abs умножить на -1
    let k = if trend.is_bull() { -1.0 } else { 1.0 };
    let delta = df.column("abs").unwrap() * k;

    // eval concrete prices from delta and current_trend_end price
    let price = trend.end().price;
    let mut price_column = delta * price / 100.0 + price;
    price_column.rename("price".into());

    df.with_column(price_column).unwrap();
    Some(df)
}

#[cached(
    ty = "SizedCache<i64, Option<DataFrame>>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ trend.begin().ts_nanos }"#
)]
fn cached_posterior_1(chart: &Chart, trend: &Trend) -> Option<DataFrame> {
    // all - all historical trends
    // obs - observation trends for current 'trend'
    // step - for this 'trend', depends on timeframe
    let all = get_trends_df(chart, trend.term()).unwrap();
    let obs = get_obs(chart, trend, &all);
    let step = get_step(chart.tf());

    // eval posterior
    let mut df = calc_posgerior(all, obs, step);

    // Если тренд бычий, значит следующий медвежий.
    // abs по модулю посчитан, так что для определения цен
    // текущего медвежьего тренда, надо abs умножить на -1
    let k = if trend.is_bull() { -1.0 } else { 1.0 };
    let delta = df.column("abs").unwrap() * k;

    // eval concrete prices from delta and current_trend_end price
    let price = trend.end().price;
    let mut price_column = delta * price / 100.0 + price;
    price_column.rename("price".into());

    df.with_column(price_column).unwrap();
    Some(df)
}
fn get_obs(chart: &Chart, trend: &Trend, all: &DataFrame) -> DataFrame {
    // stage 1 - abs size
    let value = chart.trend_abs_size(trend).unwrap().name();
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
    let value = chart.trend_speed_size(trend).unwrap().sz().name();
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
    let value = chart.trend_vol_size(trend).unwrap().sz().name();
    let obs_3 = obs_2
        .clone()
        .lazy()
        .filter(col("vol_sz").eq(lit(value)))
        .collect()
        .unwrap();

    if obs_3.height() < MIN_SAMPLE {
        obs_2
    } else {
        obs_3
    }
}
fn get_step(tf: &TimeFrame) -> f64 {
    match tf {
        TimeFrame::M1 => 0.01,
        // TimeFrame::M5 => 0.05,
        TimeFrame::M10 => 0.10,
        TimeFrame::H1 => 0.20,
        TimeFrame::Day => 0.25,
        TimeFrame::Week => 0.50,
        TimeFrame::Month => 1.00,
    }
}
fn calc_posgerior(all: DataFrame, obs: DataFrame, step: f64) -> DataFrame {
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

    while p >= MIN_P {
        combo = combo
            .lazy()
            .filter(col("id").is_in(lit(h_id.clone()), false))
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
}
