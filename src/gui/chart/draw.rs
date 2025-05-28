/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

// use egui_plot::{Line, PlotUi};
//
// use crate::{Chart, Trend};
//
// use super::palette::Palette;
//
// pub trait Draw {
//     fn draw(&self, plot: &mut PlotUi, palette: &Palette);
// }
// impl Draw for Chart {
//     fn draw(&self, plot: &mut PlotUi, palette: &Palette) {
//         for bar in self.bars().iter() {
//             // select color
//             let color = if bar.is_bull() {
//                 palette.bull
//             } else if bar.is_bear() {
//                 palette.bear
//             } else {
//                 palette.undef
//             };
//
//             // calc coordinate X
//             let x0 = bar.ts_nanos as f64;
//             let x1 = x0 + self.tf().nanos() as f64;
//             let x = (x1 + x0) / 2.0;
//
//             // create open/close/shadow lines
//             let open =
//                 Line::new("", vec![[x0, bar.o], [x, bar.o]]).color(color);
//             let close =
//                 Line::new("", vec![[x, bar.c], [x1, bar.c]]).color(color);
//             let shadow =
//                 Line::new("", vec![[x, bar.l], [x, bar.h]]).color(color);
//
//             // add lines on plot
//             plot.line(open);
//             plot.line(shadow);
//             plot.line(close);
//         }
//     }
// }
// impl Draw for Trend<'_> {
//     fn draw(&self, plot: &mut PlotUi, palette: &Palette) {
//         // select color
//         let color = palette.undef;
//
//         // calc coordinates
//         let x0 = self.begin().ts_nanos as f64;
//         let y0 = self.begin().price;
//         let x1 = self.end().ts_nanos as f64;
//         let y1 = self.end().price;
//
//         // create trend line
//         let l = Line::new("trend", vec![[x0, y0], [x1, y1]]).color(color);
//
//         // add line on plot
//         plot.line(l);
//     }
// }
