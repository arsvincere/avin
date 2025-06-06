/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Local};
use eframe::egui;
use eframe::egui::Key;
use egui_plot::Corner;
use egui_plot::Line;
use egui_plot::LineStyle;
use egui_plot::MarkerShape;
use egui_plot::Plot;
use egui_plot::PlotPoint;
use egui_plot::PlotUi;
use egui_plot::Points;

use crate::Asset;
use crate::Chart;
use crate::ChartFeatures;
use crate::Term::{self, T1, T2, T3, T4, T5};
use crate::TimeFrame;
use crate::utils;

use super::palette::Palette;

pub struct ChartWidget {
    scale_x: bool,
    scale_y: bool,
    palette: Palette,
    tf: TimeFrame,
    bars: bool,
    t1: bool,
    t2: bool,
    t3: bool,
    t4: bool,
    t5: bool,
}
impl Default for ChartWidget {
    fn default() -> Self {
        Self {
            scale_x: true,
            scale_y: true,
            palette: Palette::default(),
            tf: TimeFrame::Day,
            bars: true,
            t1: false,
            t2: false,
            t3: false,
            t4: false,
            t5: false,
        }
    }
}
impl ChartWidget {
    // build
    pub fn new() -> Self {
        // TODO: save/load state
        ChartWidget::default()
    }

    // pub
    pub fn ui(&mut self, ui: &mut egui::Ui, asset: Option<&mut Asset>) {
        self.show_toolbar(ui);

        match asset {
            Some(asset) => self.show_chart(ui, asset),
            None => self.show_empty(ui),
        };
    }

    // private
    fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // timeframes
            ui.selectable_value(&mut self.tf, TimeFrame::M1, "1M");
            ui.selectable_value(&mut self.tf, TimeFrame::M10, "10M");
            ui.selectable_value(&mut self.tf, TimeFrame::H1, "1H");
            ui.selectable_value(&mut self.tf, TimeFrame::Day, "D");
            ui.selectable_value(&mut self.tf, TimeFrame::Week, "W");
            ui.selectable_value(&mut self.tf, TimeFrame::Month, "M");
            ui.separator();

            // show bars
            if ui.selectable_label(self.bars, "Bar").clicked() {
                self.bars = !self.bars;
            };
            ui.separator();

            // show trends
            if ui.selectable_label(self.t1, "T1").clicked() {
                self.t1 = !self.t1
            };
            if ui.selectable_label(self.t2, "T2").clicked() {
                self.t2 = !self.t2;
            };
            if ui.selectable_label(self.t3, "T3").clicked() {
                self.t3 = !self.t3;
            };
            if ui.selectable_label(self.t4, "T4").clicked() {
                self.t4 = !self.t4;
            };
            if ui.selectable_label(self.t5, "T5").clicked() {
                self.t5 = !self.t5;
            };
            ui.separator();

            // let img = egui::Image::new(egui::include_image!(
            //     "../../../res/icon/btn/bar.svg"
            // ));
            // let btn = egui::Button::image(img);
            // ui.add_sized(BTN_SIZE, btn);
            // ui.separator();
        });
    }
    fn show_empty(&self, ui: &mut egui::Ui) {
        Plot::new("chart_plot")
            .show_grid(false)
            .show(ui, |_plot_ui| {});
    }
    fn show_chart(&mut self, ui: &mut egui::Ui, asset: &mut Asset) {
        let chart = get_chart(asset, &self.tf);

        let _ = ui.input(|i| {
            i.events.iter().find_map(|e| match e {
                egui::Event::Key {
                    key,
                    physical_key: _,
                    pressed,
                    repeat: _,
                    modifiers: _,
                } => {
                    if *key == Key::D && *pressed {
                        self.scale_x = true;
                        self.scale_y = false;
                    } else if *key == Key::F && *pressed {
                        self.scale_x = false;
                        self.scale_y = true;
                    } else {
                        self.scale_x = true;
                        self.scale_y = true;
                    };
                    Some(())
                }
                _ => None,
            })
        });

        let _plot = Plot::new("chart_plot")
            .show_grid(false)
            .show_axes([false, false])
            .allow_zoom([self.scale_x, self.scale_y])
            .cursor_color(self.palette.cross)
            .coordinates_formatter(Corner::LeftTop, bar_info(chart))
            .label_formatter(|name, value| price_info(chart, name, value))
            .show(ui, |plot_ui| self.draw_all(plot_ui, chart));
    }

    fn draw_all(&self, plot_ui: &mut PlotUi, chart: &Chart) {
        // draw bars
        if self.bars {
            draw_bars(plot_ui, &self.palette, chart);
        }

        // draw trends
        if self.t1 {
            draw_trends(plot_ui, &self.palette, chart, &T1);
            draw_posterior_1(plot_ui, &self.palette, chart, 1, &T1);
            draw_posterior_0(plot_ui, &self.palette, chart, 0, &T1);
        }
        if self.t2 {
            draw_trends(plot_ui, &self.palette, chart, &T2);
            draw_posterior_1(plot_ui, &self.palette, chart, 1, &T2);
            draw_posterior_0(plot_ui, &self.palette, chart, 0, &T2);
        }
        if self.t3 {
            draw_trends(plot_ui, &self.palette, chart, &T3);
            draw_posterior_1(plot_ui, &self.palette, chart, 1, &T3);
            draw_posterior_0(plot_ui, &self.palette, chart, 0, &T3);
        }
        if self.t4 {
            draw_trends(plot_ui, &self.palette, chart, &T4);
            draw_posterior_1(plot_ui, &self.palette, chart, 1, &T4);
            draw_posterior_0(plot_ui, &self.palette, chart, 0, &T4);
        }
        if self.t5 {
            draw_trends(plot_ui, &self.palette, chart, &T5);
            draw_posterior_1(plot_ui, &self.palette, chart, 1, &T5);
            draw_posterior_0(plot_ui, &self.palette, chart, 0, &T5);
        }
    }
}

fn get_chart<'a>(asset: &'a mut Asset, tf: &TimeFrame) -> &'a mut Chart {
    let loaded = asset.chart(tf).is_some();

    match loaded {
        true => asset.chart_mut(tf).unwrap(),
        false => {
            asset.load_chart(tf).unwrap();
            let chart = asset.chart_mut(tf).unwrap();
            chart.features(ChartFeatures::Extremum, true);
            chart.features(ChartFeatures::Posterior, true);

            chart
        }
    }
}
fn bar_info(chart: &Chart) -> egui_plot::CoordinatesFormatter {
    egui_plot::CoordinatesFormatter::new(|point, _bounds| {
        let nanos = point.x as i64;
        let utc = DateTime::from_timestamp_nanos(nanos);
        let dt: DateTime<Local> = DateTime::from(utc);
        let dt = dt.format("%Y-%m-%d %H:%M %a").to_string();

        let bar_opt = chart.bar(nanos);
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
fn price_info(chart: &Chart, name: &str, value: &PlotPoint) -> String {
    let step = chart.iid().step();
    let price = utils::round_price(value.y, step);

    match chart.last_price() {
        Some(last_price) => {
            let pct = (price - last_price) / last_price * 100.0;
            format!(" {}\n {:.2}%\n {}", name, pct, price)
        }
        None => {
            format!(" {}\n __.__%\n {}", name, price)
        }
    }
}
fn draw_bars(plot: &mut PlotUi, palette: &Palette, chart: &Chart) {
    for bar in chart.bars().iter() {
        // select color
        let color = if bar.is_bull() {
            palette.bull
        } else if bar.is_bear() {
            palette.bear
        } else {
            palette.undef
        };

        // eval coordinate X
        let x0 = bar.ts_nanos as f64;
        let x1 = x0 + chart.tf().nanos() as f64;
        let x = (x1 + x0) / 2.0;

        // create open/close/shadow lines
        let open = Line::new("", vec![[x0, bar.o], [x, bar.o]]).color(color);
        let close = Line::new("", vec![[x, bar.c], [x1, bar.c]]).color(color);
        let shadow = Line::new("", vec![[x, bar.l], [x, bar.h]]).color(color);

        // add lines on plot
        plot.line(open);
        plot.line(shadow);
        plot.line(close);
    }

    if let Some(bar) = chart.now() {
        // select color
        let color = if bar.is_bull() {
            palette.bull
        } else if bar.is_bear() {
            palette.bear
        } else {
            palette.undef
        };

        // eval coordinate X
        let x0 = bar.ts_nanos as f64;
        let x1 = x0 + chart.tf().nanos() as f64;
        let x = (x1 + x0) / 2.0;

        // create open/close/shadow lines
        let open = Line::new("", vec![[x0, bar.o], [x, bar.o]]).color(color);
        let close = Line::new("", vec![[x, bar.c], [x1, bar.c]]).color(color);
        let shadow = Line::new("", vec![[x, bar.l], [x, bar.h]]).color(color);

        // add lines on plot
        plot.line(open);
        plot.line(shadow);
        plot.line(close);
    }
}
fn draw_trends(
    plot: &mut PlotUi,
    palette: &Palette,
    chart: &Chart,
    term: &Term,
) {
    // draw real-time trend dashed line
    let mut n = 0;
    if let Some(trend) = chart.trend(term, n) {
        // select color
        let color = match term {
            T1 => palette.t1,
            T2 => palette.t2,
            T3 => palette.t3,
            T4 => palette.t4,
            T5 => palette.t5,
        };

        // eval coordinates
        let x0 =
            trend.begin().ts_nanos as f64 + chart.tf().nanos() as f64 / 2.0;
        let y0 = trend.begin().price;
        let x1 =
            trend.end().ts_nanos as f64 + chart.tf().nanos() as f64 / 2.0;
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
    while let Some(trend) = chart.trend(term, n) {
        // select color
        let color = match term {
            T1 => palette.t1,
            T2 => palette.t2,
            T3 => palette.t3,
            T4 => palette.t4,
            T5 => palette.t5,
        };

        // eval coordinates
        let x0 =
            trend.begin().ts_nanos as f64 + chart.tf().nanos() as f64 / 2.0;
        let y0 = trend.begin().price;
        let x1 =
            trend.end().ts_nanos as f64 + chart.tf().nanos() as f64 / 2.0;
        let y1 = trend.end().price;

        // create trend line
        let info = format!("{}", trend);
        let l = Line::new(info, vec![[x0, y0], [x1, y1]]).color(color);

        // add line on plot
        plot.line(l);

        n += 1;
    }
}
fn draw_posterior_1(
    plot: &mut PlotUi,
    palette: &Palette,
    chart: &Chart,
    n: usize,
    term: &Term,
) {
    // select color
    let color = match term {
        T1 => palette.t1,
        T2 => palette.t2,
        T3 => palette.t3,
        T4 => palette.t4,
        T5 => palette.t5,
    };

    // trend
    let trend = match chart.trend(&term, n) {
        Some(t) => t,
        None => return,
    };

    // posterior
    let p = match chart.posterior(&term, n) {
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
    let x0 = (trend.end().ts_nanos + chart.tf().nanos() / 2) as f64;
    let y0 = trend.end().price;
    let shift = chart.tf().nanos() * median;
    let x1 = (trend.end().ts_nanos + shift) as f64;
    let y1 = p.column("price").unwrap().f64().unwrap().last().unwrap();

    // line equation
    let (a, b) = solve(x0, y0, x1, y1);

    // create points
    let prices = p.column("price").unwrap().f64().unwrap();
    let mut abs = p.column("abs").unwrap().f64().unwrap().into_no_null_iter();
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
fn draw_posterior_0(
    plot: &mut PlotUi,
    palette: &Palette,
    chart: &Chart,
    n: usize,
    term: &Term,
) {
    // select color
    let color = match term {
        T1 => palette.t1, // .gamma_multiply(0.5),
        T2 => palette.t2, // .gamma_multiply(0.5),
        T3 => palette.t3, // .gamma_multiply(0.5),
        T4 => palette.t4, // .gamma_multiply(0.5),
        T5 => palette.t5, // .gamma_multiply(0.5),
    };

    // trend
    let trend = match chart.trend(term, n) {
        Some(t) => t,
        None => return,
    };

    // posterior
    let p = match chart.posterior(term, n) {
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
    let x0 = (trend.end().ts_nanos + chart.tf().nanos() / 2) as f64;
    let y0 = trend.end().price;
    let shift = chart.tf().nanos() * median;
    let x1 = (trend.end().ts_nanos + shift) as f64;
    let y1 = p.column("price").unwrap().f64().unwrap().last().unwrap();

    // line equation
    let (a, b) = solve(x0, y0, x1, y1);

    // create points
    let prices = p.column("price").unwrap().f64().unwrap();
    let mut abs = p.column("abs").unwrap().f64().unwrap().into_no_null_iter();
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
