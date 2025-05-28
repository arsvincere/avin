/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::{Analytic, TrendAnalytic, utils};

fn main() {
    log::set_logger(&utils::LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");

    // let share = Share::new("MOEX_SHARE_GAZP").unwrap();
    // let tf = TimeFrame::Day;
    // TrendAnalytic::analyse(share.iid(), &tf).unwrap();

    TrendAnalytic::analyse_all().unwrap();

    // let share = Share::new("MOEX_SHARE_GAZP").unwrap();
    // // let begin = Usr::date("2024-01-01");
    // // let end = Usr::date("2025-01-01");
    // let tf = TimeFrame::M1;
    // let term = Term::T1;
    // // let n = 1;
    // // TrendAnalytic::analyse(share.iid(), &tf).unwrap();
    //
    // // let mut chart = Chart::load(share.iid(), &tf, &begin, &end).unwrap();
    // // chart.features(avin::ChartFeatures::Extremum, true);
    // // let trend = chart.trend(&term, n).unwrap();
    // // let cdf = TrendAnalytic::abs_cdf(&trend);
    // // dbg!(&cdf);
    //
    // let name = format!("{} {} {}", TrendAnalytic::name(), tf, term);
    // let t = Timer::new();
    // for _ in 0..100 {
    //     let _df = TrendAnalytic::load(share.iid(), &name).unwrap();
    // }
    // t.stop("");
}

// Timer 1: 3.73778773s
