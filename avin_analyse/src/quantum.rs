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
    Footprint, Iid, Manager, MarketData, Quant, Quantum, Share, Source, Tic,
    TimeFrame,
};
use avin_utils::AvinError;

use crate::Analyse;

const NAME: &str = "quantum";

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

impl Analyse for Quantum {
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

    log::info!("Analyse quantum {} {}", iid.ticker(), tf);

    let quantum_df = create_quantum_df(iid, tf);

    // analyse
    analyse_quantum_feat(iid, tf, Feat::VolBuy, &quantum_df);
    analyse_quantum_feat(iid, tf, Feat::VolSell, &quantum_df);

    Ok(())
}
fn analyse_name(
    tf: TimeFrame,
    feat: Option<Feat>,
    metric: Option<Metric>,
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
fn create_quantum_df(iid: &Iid, tf: TimeFrame) -> DataFrame {
    log::info!("Create quantum df {} {}", iid, tf);

    let md = MarketData::TIC;
    let mut quantum_df = DataFrame::empty_with_schema(&Quantum::schema());
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

        let clusters = footprint.clusters();
        for cluster in clusters.iter() {
            quantum_df.extend(&cluster.quantum.df()).unwrap();
        }
        day = day.checked_add_days(Days::new(1)).unwrap();
    }

    let name = analyse_name(tf, None, None);
    Quantum::save(iid, &name, &mut quantum_df);

    quantum_df
}
fn analyse_quantum_feat(
    iid: &Iid,
    tf: TimeFrame,
    feat: Feat,
    quantum_df: &DataFrame,
) {
    log::info!("Analyse feat {}", feat.name());

    let metric = Metric::Cdf;
    let name = analyse_name(tf, Some(feat), Some(metric));
    let mut cdf = Quantum::eval_cdf(
        quantum_df
            .column(feat.name())
            .unwrap()
            .as_materialized_series(),
    );
    Quantum::save(iid, &name, &mut cdf);

    let metric = Metric::Size;
    let name = analyse_name(tf, Some(feat), Some(metric));
    let mut sizes = Quantum::eval_size(&cdf);
    Quantum::save(iid, &name, &mut sizes);

    let metric = Metric::Sz;
    let name = analyse_name(tf, Some(feat), Some(metric));
    let mut szs = Quantum::eval_sz(&cdf);
    Quantum::save(iid, &name, &mut szs);
}
fn get_cdf_df(chart: &Footprint, feat: Feat) -> Result<DataFrame, AvinError> {
    // df:
    // ┌────────┬───────┬──────────┬──────────┬───────────┐
    // │ value  ┆ count ┆ pf       ┆ cdf      ┆ cdf_p     │
    // │ ---    ┆ ---   ┆ ---      ┆ ---      ┆ ---       │
    // │ u64    ┆ u32   ┆ f64      ┆ f64      ┆ f64       │
    // ╞════════╪═══════╪══════════╪══════════╪═══════════╡
    // │ 0      ┆ 41540 ┆ 0.116884 ┆ 0.116884 ┆ 11.688403 │
    // │ 1      ┆ 4116  ┆ 0.011581 ┆ 0.128466 ┆ 12.846551 │
    // │ 2      ┆ 2256  ┆ 0.006348 ┆ 0.134813 ┆ 13.481338 │
    // │ 3      ┆ 1330  ┆ 0.003742 ┆ 0.138556 ┆ 13.855569 │
    // │ 4      ┆ 969   ┆ 0.002727 ┆ 0.141282 ┆ 14.128224 │
    // │ …      ┆ …     ┆ …        ┆ …        ┆ …         │
    // │ 204267 ┆ 1     ┆ 0.000003 ┆ 0.999989 ┆ 99.998874 │
    // │ 207767 ┆ 1     ┆ 0.000003 ┆ 0.999992 ┆ 99.999156 │
    // │ 240810 ┆ 1     ┆ 0.000003 ┆ 0.999994 ┆ 99.999437 │
    // │ 278492 ┆ 1     ┆ 0.000003 ┆ 0.999997 ┆ 99.999719 │
    // │ 359919 ┆ 1     ┆ 0.000003 ┆ 1.0      ┆ 100.0     │
    // └────────┴───────┴──────────┴──────────┴───────────┘

    let iid = chart.iid();
    let tf = chart.tf();
    let metric = Metric::Cdf;
    let name = analyse_name(tf, Some(feat), Some(metric));

    Quantum::load(iid, &name)
}

// public interface for Footprint
pub trait QuantumAnalytic {
    fn init(&mut self);
    fn quant_cdf(&self, quant: &Quant) -> Option<(f64, f64)>;
}
impl QuantumAnalytic for Footprint {
    fn init(&mut self) {}
    fn quant_cdf(&self, quant: &Quant) -> Option<(f64, f64)> {
        let cdf_b = match get_cdf_df(self, Feat::VolBuy) {
            Ok(cdf_df) => Quantum::cdf(quant.vol_b, cdf_df),
            Err(_) => -1.0,
        };
        let cdf_s = match get_cdf_df(self, Feat::VolSell) {
            Ok(cdf_df) => Quantum::cdf(quant.vol_s, cdf_df),
            Err(_) => -1.0,
        };

        if cdf_b > 0.0 && cdf_s > 0.0 {
            return Some((cdf_b, cdf_s));
        }

        None
    }
}
