use avin::*;

#[tokio::main]
async fn main() {
    let mut asset = Asset::from("moex_share_sber").unwrap();
    let tf = TimeFrame::new("5M");

    let chart = asset.load_chart(&tf).unwrap();
    assert_eq!(chart.tf(), &tf);

    dbg!(chart.bars().len());
    dbg!(chart.first());
    dbg!(chart.last());
    assert!(chart.bars().len() > 4000);
    assert!(chart.bars().len() < 5000);
}

// Request bars: 944.485158ms  - collect
// Request bars: 785.288914ms  - iter
