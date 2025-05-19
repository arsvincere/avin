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
use strum::IntoEnumIterator;

use crate::Chart;
use crate::Features;
use crate::Share;
use crate::Term;
use crate::TimeFrame;
use crate::Trend;

use super::Analytic;

pub struct TrendAnalytic {}
impl TrendAnalytic {}
impl Analytic for TrendAnalytic {
    fn name() -> &'static str {
        "trend"
    }
    fn analyse(share: &Share, tf: &TimeFrame) -> Result<(), String> {
        log::info!(":: Analyse trend {} {}", share.ticker(), tf);

        let mut chart = load_chart(share, tf).unwrap();
        chart.features(Features::Extremum, true);

        for term in Term::iter() {
            let trends = chart.all_trends(&term);
            let mut df = create_df(&trends);
            let name = format!("{} {} {}", TrendAnalytic::name(), tf, term);

            TrendAnalytic::save(share, &name, &mut df);
        }

        Ok(())
    }
    fn analyse_all() -> Result<(), String> {
        let shares = Share::all();
        let timeframes = TimeFrame::all();

        for share in shares.iter() {
            for tf in timeframes.iter() {
                TrendAnalytic::analyse(share, tf).unwrap();
            }
        }
        Ok(())
    }
}

fn load_chart(share: &Share, tf: &TimeFrame) -> Result<Chart, String> {
    log::info!("   Load chart");

    let begin = Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap();
    let end = Utc::now();

    let chart = Chart::load(share.iid(), tf, &begin, &end).unwrap();

    Ok(chart)
}
fn create_df(trends: &Vec<Trend>) -> DataFrame {
    log::info!("   Create df");

    let mut begin = Vec::new();
    let mut end = Vec::new();
    let mut begin_price = Vec::new();
    let mut end_price = Vec::new();
    let mut kind = Vec::new();
    let mut period = Vec::new();
    let mut abs = Vec::new();
    let mut speed = Vec::new();
    let mut vol_bear = Vec::new();
    let mut vol_bull = Vec::new();
    let mut vol_total = Vec::new();

    for trend in trends.iter() {
        begin.push(trend.begin().ts_nanos);
        end.push(trend.end().ts_nanos);
        begin_price.push(trend.begin().price);
        end_price.push(trend.end().price);
        kind.push(if trend.is_bull() { "Bull" } else { "Bear" });
        period.push(trend.period());
        abs.push(trend.abs_p());
        speed.push(trend.speed_p());
        vol_bear.push(trend.vol_bear());
        vol_bull.push(trend.vol_bull());
        vol_total.push(trend.vol_total());
    }

    let df = df!(
            "begin" => begin,
            "end" => end,
            "begin_price" => begin_price,
            "end_price" => end_price,
            "kind" => kind,
            "period" => period,
            "abs" => abs,
            "speed" => speed,
            "vol_bear" => vol_bear,
            "vol_bull" => vol_bull,
            "vol_total" => vol_total,
    );

    df.unwrap()
}
