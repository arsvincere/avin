/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

// use chrono::{Days, NaiveDate, NaiveTime, Utc};
// use polars::prelude::*;
// use strum::EnumIter;
//
// use avin::core::{
//     Cluster, Footprint, Iid, Manager, MarketData, Quant, Quantum, Share, Tic,
//     TimeFrame,
// };
//
// use crate::Analyse;
//
// pub struct ClusterAnalytic {}
// impl ClusterAnalytic {
//     pub fn quant_cdf(
//         footprint: &Footprint,
//         quant: &Quant,
//     ) -> Option<(f64, f64)> {
//         let cdf_b = match get_cdf_df(footprint, &Feat::VolBuy) {
//             Ok(cdf_df) => Self::cdf(quant.vol_b, cdf_df),
//             Err(_) => -1.0,
//         };
//         let cdf_s = match get_cdf_df(footprint, &Feat::VolSell) {
//             Ok(cdf_df) => Self::cdf(quant.vol_s, cdf_df),
//             Err(_) => -1.0,
//         };
//
//         if cdf_b > 0.0 && cdf_s > 0.0 {
//             return Some((cdf_b, cdf_s));
//         } else {
//             return None;
//         }
//     }
// }
// impl Analytic for ClusterAnalytic {
//     #[inline]
//     fn name() -> &'static str {
//         "cluster"
//     }
//     fn analyse(iid: &Iid, tf: &TimeFrame) -> Result<(), String> {
//         log::info!(":: Analyse {} {} {}", Self::name(), iid.ticker(), tf);
//
//         create_clusters_df(iid, tf);
//         analyse_quantum(iid, tf);
//
//         Ok(())
//     }
//     fn analyse_all() -> Result<(), String> {
//         let shares = Share::all();
//         let timeframes = vec![
//             TimeFrame::M1,
//             TimeFrame::M10,
//             TimeFrame::H1,
//             TimeFrame::Day,
//         ];
//
//         for share in shares.iter() {
//             for tf in timeframes.iter() {
//                 Self::analyse(share.iid(), tf).unwrap();
//             }
//         }
//
//         Ok(())
//     }
// }
//
// #[derive(Debug, EnumIter)]
// enum Aspect {
//     Cluster,
//     Quantum,
// }
// impl Aspect {
//     #[inline]
//     fn name(&self) -> &str {
//         match self {
//             Self::Cluster => "cluster",
//             Self::Quantum => "quantum",
//         }
//     }
// }
//
// #[derive(Debug, EnumIter)]
// enum Feat {
//     VolBuy,
//     VolSell,
// }
// impl Feat {
//     #[inline]
//     fn name(&self) -> &str {
//         match self {
//             Self::VolBuy => "vol_b",
//             Self::VolSell => "vol_s",
//         }
//     }
// }
//
// #[derive(Debug, EnumIter)]
// enum Metric {
//     CDF,
//     Size,
//     Sz,
// }
// impl Metric {
//     #[inline]
//     fn name(&self) -> &str {
//         match self {
//             Self::CDF => "cdf",
//             Self::Size => "size",
//             Self::Sz => "sz",
//         }
//     }
// }
//
// fn analyse_name(
//     tf: &TimeFrame,
//     aspect: &Aspect,
//     feat: Option<&Feat>,
//     metric: Option<&Metric>,
// ) -> String {
//     if feat.is_some() && metric.is_some() {
//         format!(
//             "{} {} {} {} {}",
//             ClusterAnalytic::name(),
//             tf,
//             aspect.name(),
//             feat.unwrap().name(),
//             metric.unwrap().name(),
//         )
//     } else {
//         format!("{} {} {}", ClusterAnalytic::name(), tf, aspect.name())
//     }
// }
// fn get_period() -> (NaiveDate, NaiveDate) {
//     let begin = NaiveDate::from_ymd_opt(2025, 3, 7).unwrap();
//     let end = Utc::now().date_naive();
//
//     (begin, end)
// }
// fn create_clusters_df(iid: &Iid, tf: &TimeFrame) {
//     log::info!("Create cluster df {} {}", iid, tf);
//
//     let md = MarketData::TIC;
//     let mut clusters_df = DataFrame::empty_with_schema(&Cluster::schema());
//     let mut quantum_df = DataFrame::empty_with_schema(&Quantum::schema());
//     let (mut day, end) = get_period();
//     while day < end {
//         let b = day.and_time(NaiveTime::MIN).and_utc();
//         let e = day
//             .checked_add_days(Days::new(1))
//             .unwrap()
//             .and_time(NaiveTime::MIN)
//             .and_utc();
//         let tic_file = match Manager::load(iid, &md, &b, &e) {
//             Ok(data) => data,
//             Err(_) => {
//                 log::warn!("no tics for {} {}", iid.ticker(), day);
//                 day = day.checked_add_days(Days::new(1)).unwrap();
//                 continue;
//             }
//         };
//
//         let tics = Tic::from_df(tic_file.df()).unwrap();
//
//         let footprint = Footprint::from_tics(iid, tf, &tics);
//
//         let cluster = footprint.clusters();
//         for c in cluster.iter() {
//             clusters_df.extend(&c.df()).unwrap();
//             quantum_df.extend(&c.quantum.df()).unwrap();
//         }
//         day = day.checked_add_days(Days::new(1)).unwrap();
//     }
//
//     let name = analyse_name(tf, &Aspect::Cluster, None, None);
//     ClusterAnalytic::save(iid, &name, &mut clusters_df);
//     let name = analyse_name(tf, &Aspect::Quantum, None, None);
//     ClusterAnalytic::save(iid, &name, &mut quantum_df);
// }
// fn analyse_quantum(iid: &Iid, tf: &TimeFrame) {
//     log::info!("Analyse quantum {} {}", iid.ticker(), tf);
//
//     // load all quantum
//     let name = analyse_name(tf, &Aspect::Quantum, None, None);
//     let quantum_df = ClusterAnalytic::load(iid, &name).unwrap();
//
//     // analyse
//     analyse_quantum_feat(iid, tf, &Feat::VolBuy, &quantum_df);
//     analyse_quantum_feat(iid, tf, &Feat::VolSell, &quantum_df);
// }
// fn analyse_quantum_feat(
//     iid: &Iid,
//     tf: &TimeFrame,
//     feat: &Feat,
//     quantum_df: &DataFrame,
// ) {
//     log::info!("Analyse feat {}", feat.name());
//
//     let metric = Metric::CDF;
//     let name = analyse_name(tf, &Aspect::Quantum, Some(feat), Some(&metric));
//     let mut cdf = ClusterAnalytic::eval_cdf(
//         quantum_df
//             .column(feat.name())
//             .unwrap()
//             .as_materialized_series(),
//     );
//     ClusterAnalytic::save(iid, &name, &mut cdf);
//
//     let metric = Metric::Size;
//     let name = analyse_name(tf, &Aspect::Quantum, Some(feat), Some(&metric));
//     let mut sizes = ClusterAnalytic::eval_size(&cdf);
//     ClusterAnalytic::save(iid, &name, &mut sizes);
//
//     let metric = Metric::Sz;
//     let name = analyse_name(tf, &Aspect::Quantum, Some(feat), Some(&metric));
//     let mut szs = ClusterAnalytic::eval_sz(&cdf);
//     ClusterAnalytic::save(iid, &name, &mut szs);
// }
// fn get_cdf_df(chart: &Footprint, feat: &Feat) -> Result<DataFrame, String> {
//     // df:
//     // ┌────────┬───────┬──────────┬──────────┬───────────┐
//     // │ value  ┆ count ┆ pf       ┆ cdf      ┆ cdf_p     │
//     // │ ---    ┆ ---   ┆ ---      ┆ ---      ┆ ---       │
//     // │ u64    ┆ u32   ┆ f64      ┆ f64      ┆ f64       │
//     // ╞════════╪═══════╪══════════╪══════════╪═══════════╡
//     // │ 0      ┆ 41540 ┆ 0.116884 ┆ 0.116884 ┆ 11.688403 │
//     // │ 1      ┆ 4116  ┆ 0.011581 ┆ 0.128466 ┆ 12.846551 │
//     // │ 2      ┆ 2256  ┆ 0.006348 ┆ 0.134813 ┆ 13.481338 │
//     // │ 3      ┆ 1330  ┆ 0.003742 ┆ 0.138556 ┆ 13.855569 │
//     // │ 4      ┆ 969   ┆ 0.002727 ┆ 0.141282 ┆ 14.128224 │
//     // │ …      ┆ …     ┆ …        ┆ …        ┆ …         │
//     // │ 204267 ┆ 1     ┆ 0.000003 ┆ 0.999989 ┆ 99.998874 │
//     // │ 207767 ┆ 1     ┆ 0.000003 ┆ 0.999992 ┆ 99.999156 │
//     // │ 240810 ┆ 1     ┆ 0.000003 ┆ 0.999994 ┆ 99.999437 │
//     // │ 278492 ┆ 1     ┆ 0.000003 ┆ 0.999997 ┆ 99.999719 │
//     // │ 359919 ┆ 1     ┆ 0.000003 ┆ 1.0      ┆ 100.0     │
//     // └────────┴───────┴──────────┴──────────┴───────────┘
//
//     let iid = chart.iid();
//     let tf = chart.tf();
//     let metric = Metric::CDF;
//     let name = analyse_name(tf, &Aspect::Quantum, Some(&feat), Some(&metric));
//
//     ClusterAnalytic::load(iid, &name)
// }
