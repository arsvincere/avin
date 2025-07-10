use avin::analyse::*;
use avin::core::*;
use avin::utils;

fn main() {
    utils::init_logger();

    Trend::analyse_all().unwrap();
    Bar::analyse_all().unwrap();
    // ClusterAnalytic::analyse_all().unwrap();
}
