use avin::utils;
use avin_data::*;

#[tokio::main]
async fn main() {
    utils::init_logger();

    let source = Source::MOEX;
    let iid = Manager::find_iid("moex_share_sber").unwrap();
    let md = MarketData::OB_STATS;
    let year = Some(2020);

    Manager::download(source, &iid, md, year).await.unwrap();
}
