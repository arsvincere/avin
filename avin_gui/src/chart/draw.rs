/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Local};
use egui_plot::{Line, LineStyle, MarkerShape, PlotPoint, PlotUi, Points};

use avin_analyse::TrendAnalytic;
use avin_core::{
    Chart, ExtremumIndicator, Footprint,
    Term::{self, T1, T2, T3, T4, T5},
};
use avin_utils as utils;

use crate::theme::Theme;

pub trait ChartDraw {
    fn bar_info(&self) -> egui_plot::CoordinatesFormatter;
    fn price_info(&self, name: &str, value: &PlotPoint) -> String;

    fn draw_bars(&self, plot: &mut PlotUi, theme: &Theme);
    fn draw_trends(&self, plot: &mut PlotUi, theme: &Theme, term: Term);

    fn draw_posterior_0(&self, plot: &mut PlotUi, theme: &Theme, term: Term);
    fn draw_posterior_1(&self, plot: &mut PlotUi, theme: &Theme, term: Term);
}
impl ChartDraw for Chart {
    fn bar_info(&self) -> egui_plot::CoordinatesFormatter {
        egui_plot::CoordinatesFormatter::new(|point, _bounds| {
            let nanos = point.x as i64;
            let utc = DateTime::from_timestamp_nanos(nanos);
            let dt: DateTime<Local> = DateTime::from(utc);
            let dt = dt.format("%Y-%m-%d %H:%M %a").to_string();

            let bar_opt = self.get_bar_of_ts(nanos);
            match bar_opt {
                None => dt,
                Some(bar) => {
                    format!(
                        "{}  O: {} H: {} L: {} C: {} Vol: {}  \
                        [Body: {}% | Full: {}%]",
                        bar.dt_local().format("%Y-%m-%d %H:%M %a"),
                        bar.o,
                        bar.h,
                        bar.l,
                        bar.c,
                        bar.v,
                        bar.full().delta_p(),
                        bar.body().delta_p()
                    )
                }
            }
        })
    }
    fn price_info(&self, name: &str, value: &PlotPoint) -> String {
        let step = self.iid().step();
        let price = utils::round_price(value.y, step);

        match self.last_price() {
            Some(last_price) => {
                let pct = (price - last_price) / last_price * 100.0;
                format!(" {}\n {:.2}%\n {}", name, pct, price)
            }
            None => {
                format!(" {}\n __.__%\n {}", name, price)
            }
        }
    }
    fn draw_bars(&self, plot: &mut PlotUi, theme: &Theme) {
        for bar in self.bars().iter() {
            // select color
            let color = if bar.is_bull() {
                theme.bull
            } else if bar.is_bear() {
                theme.bear
            } else {
                theme.undef
            };

            // eval coordinate X
            let x0 = bar.ts_nanos as f64;
            let x1 = x0 + self.tf().nanos() as f64;
            let x = (x1 + x0) / 2.0;

            // create open/close/shadow lines
            let open =
                Line::new("", vec![[x0, bar.o], [x, bar.o]]).color(color);
            let close =
                Line::new("", vec![[x, bar.c], [x1, bar.c]]).color(color);
            let shadow =
                Line::new("", vec![[x, bar.l], [x, bar.h]]).color(color);

            // add lines on plot
            plot.line(open);
            plot.line(shadow);
            plot.line(close);
        }

        if let Some(bar) = self.now() {
            // select color
            let color = if bar.is_bull() {
                theme.bull
            } else if bar.is_bear() {
                theme.bear
            } else {
                theme.undef
            };

            // eval coordinate X
            let x0 = bar.ts_nanos as f64;
            let x1 = x0 + self.tf().nanos() as f64;
            let x = (x1 + x0) / 2.0;

            // create open/close/shadow lines
            let open =
                Line::new("", vec![[x0, bar.o], [x, bar.o]]).color(color);
            let close =
                Line::new("", vec![[x, bar.c], [x1, bar.c]]).color(color);
            let shadow =
                Line::new("", vec![[x, bar.l], [x, bar.h]]).color(color);

            // add lines on plot
            plot.line(open);
            plot.line(shadow);
            plot.line(close);
        }
    }
    fn draw_trends(&self, plot: &mut PlotUi, theme: &Theme, term: Term) {
        // draw real-time trend dashed line
        let mut n = 0;
        if let Some(trend) = self.trend(term, n) {
            // select color
            let color = match term {
                T1 => theme.t1,
                T2 => theme.t2,
                T3 => theme.t3,
                T4 => theme.t4,
                T5 => theme.t5,
            };

            // eval coordinates
            let x0 = trend.begin().ts_nanos as f64
                + self.tf().nanos() as f64 / 2.0;
            let y0 = trend.begin().price;
            let x1 =
                trend.end().ts_nanos as f64 + self.tf().nanos() as f64 / 2.0;
            let y1 = trend.end().price;

            // create trend line
            let info = format!("{}", trend);
            let l = Line::new(info, vec![[x0, y0], [x1, y1]])
                .color(color)
                .style(LineStyle::Dashed { length: 10.0 });

            // add line on plot
            plot.line(l);

            n += 1;
        }

        // draw other trends solid line
        while let Some(trend) = self.trend(term, n) {
            // select color
            let color = match term {
                T1 => theme.t1,
                T2 => theme.t2,
                T3 => theme.t3,
                T4 => theme.t4,
                T5 => theme.t5,
            };

            // eval coordinates
            let x0 = trend.begin().ts_nanos as f64
                + self.tf().nanos() as f64 / 2.0;
            let y0 = trend.begin().price;
            let x1 =
                trend.end().ts_nanos as f64 + self.tf().nanos() as f64 / 2.0;
            let y1 = trend.end().price;

            // create trend line
            let info = format!("{}", trend);
            let l = Line::new(info, vec![[x0, y0], [x1, y1]]).color(color);

            // add line on plot
            plot.line(l);

            n += 1;
        }
    }

    fn draw_posterior_0(&self, plot: &mut PlotUi, theme: &Theme, term: Term) {
        // trend number for posterior
        let n = 0;

        // select color
        let color = match term {
            T1 => theme.t1, // .gamma_multiply(0.5),
            T2 => theme.t2, // .gamma_multiply(0.5),
            T3 => theme.t3, // .gamma_multiply(0.5),
            T4 => theme.t4, // .gamma_multiply(0.5),
            T5 => theme.t5, // .gamma_multiply(0.5),
        };

        // trend
        let trend = match self.trend(term, n) {
            Some(t) => t,
            None => return,
        };

        // posterior
        let p = match self.trend_posterior_now(term) {
            Some(p) => p,
            None => return,
        };

        // get median len
        let median = match term {
            T1 => 3,
            T2 => 6,
            T3 => 12,
            T4 => 24,
            T5 => 48,
        };

        // eval coordinates
        let x0 = (trend.end().ts_nanos + self.tf().nanos() / 2) as f64;
        let y0 = trend.end().price;
        let shift = self.tf().nanos() * median;
        let x1 = (trend.end().ts_nanos + shift) as f64;
        let y1 = p.column("price").unwrap().f64().unwrap().last().unwrap();

        // line equation
        let (a, b) = solve(x0, y0, x1, y1);

        // create points
        let prices = p.column("price").unwrap().f64().unwrap();
        let mut abs =
            p.column("abs").unwrap().f64().unwrap().into_no_null_iter();
        let mut p = p.column("p").unwrap().f64().unwrap().into_no_null_iter();
        for price in prices.into_no_null_iter() {
            let info = format!(
                "{}  abs={}  p={:.2}%",
                term,
                abs.next().unwrap(),
                p.next().unwrap()
            );
            let x = x(a, b, price);

            let points = Points::new(info, vec![[x, price]]).color(color);

            plot.points(points);
        }
    }
    fn draw_posterior_1(&self, plot: &mut PlotUi, theme: &Theme, term: Term) {
        // trend number for posterior
        let n = 1;

        // select color
        let color = match term {
            T1 => theme.t1,
            T2 => theme.t2,
            T3 => theme.t3,
            T4 => theme.t4,
            T5 => theme.t5,
        };

        // trend
        let trend = match self.trend(term, n) {
            Some(t) => t,
            None => return,
        };

        // posterior
        let p = match self.trend_posterior_last(term) {
            Some(p) => p,
            None => return,
        };

        // get median len
        let median = match term {
            T1 => 4,
            T2 => 8,
            T3 => 16,
            T4 => 32,
            T5 => 64,
        };

        // eval coordinates
        let x0 = (trend.end().ts_nanos + self.tf().nanos() / 2) as f64;
        let y0 = trend.end().price;
        let shift = self.tf().nanos() * median;
        let x1 = (trend.end().ts_nanos + shift) as f64;
        let y1 = p.column("price").unwrap().f64().unwrap().last().unwrap();

        // line equation
        let (a, b) = solve(x0, y0, x1, y1);

        // create points
        let prices = p.column("price").unwrap().f64().unwrap();
        let mut abs =
            p.column("abs").unwrap().f64().unwrap().into_no_null_iter();
        let mut p = p.column("p").unwrap().f64().unwrap().into_no_null_iter();
        for price in prices.into_no_null_iter() {
            let info = format!(
                "{}  abs={}  p={:.2}%",
                term,
                abs.next().unwrap(),
                p.next().unwrap()
            );
            let x = x(a, b, price);

            let points = Points::new(info, vec![[x, price]])
                .color(color)
                .filled(true)
                .radius(3.0)
                .shape(MarkerShape::Circle);

            plot.points(points);
        }
    }
}

pub trait FootprintDraw {
    fn draw_hist(&self, plot: &mut PlotUi, theme: &Theme);
    // fn draw_quantum(&self, plot: &mut PlotUi, theme: &Theme);
}
impl FootprintDraw for Footprint {
    fn draw_hist(&self, plot: &mut PlotUi, theme: &Theme) {
        for cluster in self.clusters().iter() {
            // eval coordinate X
            let x0 = cluster.ts_nanos as f64;
            let x1 = x0 + self.tf().nanos() as f64;
            let x = (x1 + x0) / 2.0;
            let y = 0.0;
            let y_buy = cluster.val_b;
            let y_sell = cluster.val_s * -1.0;

            // create buy / sell bars
            let b = Line::new("", vec![[x, y], [x, y_buy]]).color(theme.bull);
            let s =
                Line::new("", vec![[x, y], [x, y_sell]]).color(theme.bear);

            // add lines on plot
            plot.line(b);
            plot.line(s);
        }
    }
    // fn draw_quantum(plot: &mut PlotUi, theme: &Theme, footprint: &Footprint) {
    //     for cluster in footprint.clusters().iter() {
    //         for quant in cluster.quantum.quants().iter() {
    //             // eval coordinate
    //             let x0 = cluster.ts_nanos as f64;
    //             let x1 = x0 + footprint.tf().nanos() as f64;
    //             let x = (x1 + x0) / 2.0;
    //             let width = x1 - x;
    //             let y = quant.price;
    //
    //             if let Some((buy, sell)) =
    //                 ClusterAnalytic::quant_cdf(footprint, quant)
    //             {
    //                 println!(
    //                     "b={} ({})\t s={} ({})\t p={}",
    //                     buy, quant.vol_b, sell, quant.vol_s, quant.price
    //                 );
    //                 let right = x + width * buy;
    //                 let left = x - width * sell;
    //
    //                 // create buy / sell lines
    //                 let b = Line::new("", vec![[x, y], [right, y]])
    //                     .color(theme.bull);
    //                 let s = Line::new("", vec![[x, y], [left, y]])
    //                     .color(theme.bear);
    //
    //                 // add lines on plot
    //                 plot.line(b);
    //                 plot.line(s);
    //             } else {
    //                 return;
    //             }
    //         }
    //     }
    // }
}

fn solve(x0: f64, y0: f64, x1: f64, y1: f64) -> (f64, f64) {
    // y = ax + b
    //
    // y0 = ax0 + b
    // y1 = ax1 + b
    //
    // b = y0 - ax0
    //
    // y1 = ax1 + y0 - ax0
    // y1 - y0 = ax1 - ax0
    // y1 - y0 = a(x1 -x0)
    // a = (y1 - y0) / (x1 - x0)

    let a = (y1 - y0) / (x1 - x0);
    let b = y0 - a * x0;

    (a, b)
}
fn x(a: f64, b: f64, y: f64) -> f64 {
    // y = ax + b
    // ax = y - b
    // x = (y - b) / a

    (y - b) / a
}
