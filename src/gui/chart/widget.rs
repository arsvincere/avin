/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Local};
use eframe::egui;
use egui_plot::{Corner, Line, LineStyle, Plot, PlotPoint, PlotUi};

use crate::{
    Asset, Chart, ChartFeatures,
    Term::{self, T1, T2, T3, T4, T5},
    TimeFrame, utils,
};

use super::palette::Palette;

pub struct ChartWidget {
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
            if ui.selectable_label(self.bars, "Bar").clicked() {
                self.bars = !self.bars;
            };
            ui.selectable_value(&mut self.tf, TimeFrame::M1, "1M");
            ui.selectable_value(&mut self.tf, TimeFrame::M10, "10M");
            ui.selectable_value(&mut self.tf, TimeFrame::H1, "1H");
            ui.selectable_value(&mut self.tf, TimeFrame::Day, "D");
            ui.selectable_value(&mut self.tf, TimeFrame::Week, "W");
            ui.selectable_value(&mut self.tf, TimeFrame::Month, "M");
            ui.separator();

            // trends
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
    fn show_chart(&self, ui: &mut egui::Ui, asset: &mut Asset) {
        let chart = get_chart(asset, &self.tf);

        Plot::new("chart_plot")
            .show_grid(false)
            .show_axes([false, false])
            .cursor_color(self.palette.cross)
            .coordinates_formatter(Corner::LeftTop, bar_info(chart))
            .label_formatter(|name, value| price_info(chart, name, value))
            .show(ui, |plot_ui| self.draw_all(plot_ui, chart));
    }
    fn draw_all(&self, plot_ui: &mut PlotUi, chart: &Chart) {
        if self.bars {
            draw_bars(plot_ui, &self.palette, chart);
        }

        if self.t1 {
            draw_trends(plot_ui, &self.palette, chart, &T1);
        }
        if self.t2 {
            draw_trends(plot_ui, &self.palette, chart, &T2);
        }
        if self.t3 {
            draw_trends(plot_ui, &self.palette, chart, &T3);
        }
        if self.t4 {
            draw_trends(plot_ui, &self.palette, chart, &T4);
        }
        if self.t5 {
            draw_trends(plot_ui, &self.palette, chart, &T5);
        }
    }
}

fn get_chart<'a>(asset: &'a mut Asset, tf: &TimeFrame) -> &'a mut Chart {
    let loaded = asset.chart(tf).is_some();

    match loaded {
        true => asset.chart_mut(tf).unwrap(),
        false => {
            let chart = asset.load_chart_mut(tf).unwrap();
            chart.features(ChartFeatures::Extremum, true);

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
                        [Full: {}% Body: {}%]",
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

        // calc coordinate X
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

        // calc coordinates
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

        // calc coordinates
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
