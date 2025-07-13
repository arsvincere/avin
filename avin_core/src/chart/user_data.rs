/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use polars::prelude::DataFrame;

use crate::Bar;

pub trait UserData: Send {
    fn id(&self) -> &str;
    fn df(&self) -> &DataFrame;
    fn init(&mut self, bars: &[Bar], now: Option<&Bar>);
    fn update(&mut self, bars: &[Bar], now: Option<&Bar>);
}

// const ID: &str = "ExtrID";
// #[derive(Debug)]
// struct ExtremumData {
//     data: DataFrame,
// }
// impl UserData for ExtremumData {
//     fn id(&self) -> &'static str {
//         ID
//     }
//     fn data(&self) -> &DataFrame {
//         &self.data
//     }
//     fn update(&mut self, bar: i32) {
//         let d1 = df!(
//             "e1" => [44],
//             "e2" => [4],
//             "e3" => [4],
//         )
//         .unwrap();
//         println!("updated!");
//         self.data.extend(&d1).unwrap();
//     }
// }
// trait ExtremumIndicator {
//     fn extr(&mut self, i: usize) -> Option<i32>;
// }
// impl ExtremumIndicator for Chart {
//     fn extr(&mut self, i: usize) -> Option<i32> {
//         let df = match self.get_ind(ID) {
//             Some(df) => df,
//             None => {
//                 extr_init(self);
//                 self.get_ind(ID).unwrap()
//             }
//         };
//
//         println!("h={}", df.height());
//         println!("i={}", i);
//         println!("c={}", i >= df.height());
//
//         if i >= df.height() {
//             return None;
//         }
//
//         df.column("e1").unwrap().i32().unwrap().get(i)
//     }
// }
//
// fn extr_init(chart: &mut Chart) {
//     let d1 = df!(
//         "e1" => [11, 22, 33],
//         "e2" => [1, 2, 3],
//         "e3" => [1, 2, 3],
//     )
//     .unwrap();
//
//     let i1 = ExtremumData { data: d1 };
//     let i1 = Box::new(i1);
//
//     chart.add_ind(i1);
// }
