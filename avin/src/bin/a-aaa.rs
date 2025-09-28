#![allow(dead_code)]
#![allow(unused)]

use chrono::prelude::*;
use std::path::Path;

use avin_analyse::*;
use avin_core::*;
use avin_data::*;
use avin_simulator::*;
use avin_strategy::*;
use avin_utils::*;

#[tokio::main]
async fn main() {
    let path = Path::new("/home/alex/trading/usr/asset/xxx.csv");
    let asset_list = AssetList::load(path).unwrap();

    assert_eq!(asset_list.name(), "xxx");

    assert_ne!(asset_list.assets().len(), 0);
}
