use avin::analyse::*;
use avin::core::*;
use avin::utils;

fn main() {
    utils::init_logger();

    Trend::analyse_all().unwrap();
    // Bar::analyse_all().unwrap();

    // let asset = Asset::new("moex_share_afks").unwrap();
    // let iid = asset.iid();
    // let tf = TimeFrame::Day;
    // Trend::analyse(iid, &tf).unwrap();
}
