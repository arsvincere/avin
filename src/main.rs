/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * URL:         http://arsvincere.com                                      *
 * AUTHOR:      Alex Avin                                                  *
 * E-MAIL:      mr.alexavin@gmail.com                                      *
 * LICENSE:     MIT                                                        *
 *   __   _____   ____     _    _  ___  __   _  _____  _____   ____  _____ *
 *  /__\  |___/  |___       \  /    |   | \  |  |      |___   |___/  |___  *
 * |    | |  \_  ____|       \/    _|_  |  \_|  |____  |____  |  \_  |____ *
 *                                                                         *
 * * * * * * * * Open source cross-platform trading system * * * * * * * * */

#![allow(unused)]

use std::path::Path;

use avin::*;
use chrono::NaiveTime;

#[tokio::main]
async fn main() {
    let mut asset = Asset::new("moex_share_sber").unwrap();
    asset.load_tics();

    let t = utils::Timer::new();
    let f = asset.build_footprint(&TimeFrame::Day).unwrap();
    t.stop("D");
    dbg!(&f.df());

    let t = utils::Timer::new();
    let f = asset.build_footprint(&TimeFrame::H1).unwrap();
    t.stop("1H");
    dbg!(&f.df());

    let t = utils::Timer::new();
    let f = asset.build_footprint(&TimeFrame::M10).unwrap();
    t.stop("M10");
    dbg!(&f.df());

    let t = utils::Timer::new();
    let f = asset.build_footprint(&TimeFrame::M1).unwrap();
    t.stop("M1");
    dbg!(&f.df());
}
