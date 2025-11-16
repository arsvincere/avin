/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{Days, NaiveDate, NaiveTime, Utc};
use polars::prelude::*;
use strum::EnumIter;

use avin_core::{
    Cluster, Footprint, Iid, Manager, MarketData, Share, Source, Tic, TimeFrame,
};

use crate::Analyse;

const NAME: &str = "cluster";

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
    VolBuy,
    VolSell,
}
impl Feat {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::VolBuy => "vol_b",
            Self::VolSell => "vol_s",
        }
    }
}

impl Analyse for Cluster {
    fn analyse() -> Result<(), avin_utils::AvinError> {
        let shares = Share::all();
        let timeframes =
            [TimeFrame::M1, TimeFrame::M10, TimeFrame::H1, TimeFrame::Day];

        for share in shares.iter() {
            for tf in timeframes.iter() {
                analyse(share.iid(), *tf).unwrap();
            }
        }

        Ok(())
    }
}

fn analyse(iid: &Iid, tf: TimeFrame) -> Result<(), avin_utils::AvinError> {
    log::info!(":: Analyse {} {} {}", NAME, iid.ticker(), tf);

    create_clusters_df(iid, tf);

    Ok(())
}
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
fn get_period() -> (NaiveDate, NaiveDate) {
    let begin = NaiveDate::from_ymd_opt(2025, 8, 16).unwrap();
    let end = Utc::now().date_naive();

    (begin, end)
}
fn create_clusters_df(iid: &Iid, tf: TimeFrame) {
    log::info!("Create cluster df {} {}", iid, tf);

    let md = MarketData::TIC;
    let mut clusters_df = DataFrame::empty_with_schema(&Cluster::schema());
    let (mut day, end) = get_period();
    while day < end {
        let b = day.and_time(NaiveTime::MIN).and_utc();
        let e = day
            .checked_add_days(Days::new(1))
            .unwrap()
            .and_time(NaiveTime::MIN)
            .and_utc();

        let source = Source::MOEXALGO;
        let tic_df = match Manager::load(iid, source, md, b, e) {
            Ok(data) => data,
            Err(_) => {
                log::warn!("no tics for {} {}", iid.ticker(), day);
                day = day.checked_add_days(Days::new(1)).unwrap();
                continue;
            }
        };

        let tics = Tic::from_df(&tic_df).unwrap();

        let footprint = Footprint::from_tics(iid, tf, &tics);

        let cluster = footprint.clusters();
        for c in cluster.iter() {
            clusters_df.extend(&c.df()).unwrap();
        }
        day = day.checked_add_days(Days::new(1)).unwrap();
    }

    let name = analyse_name(tf, None, None);
    Cluster::save(iid, &name, &mut clusters_df);
}
