use chrono::{DateTime, Local};
use eframe::egui::{self, Color32};
use egui_plot::{Corner, Line, Plot};

use crate::{Asset, Bar, Chart, TimeFrame, utils};

use super::colors::Colors;

pub struct ChartWidget {
    tf: TimeFrame,
    colors: Colors,
}
impl Default for ChartWidget {
    fn default() -> Self {
        Self {
            tf: TimeFrame::Day,
            colors: Colors::default(),
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
    fn get_chart<'a>(&self, asset: &'a mut Asset) -> &'a mut Chart {
        let loaded = asset.chart(&self.tf).is_some();

        match loaded {
            true => asset.chart_mut(&self.tf).unwrap(),
            false => asset.load_chart_mut(&self.tf).unwrap(),
        }
    }
    fn get_color(&self, bar: &Bar) -> Color32 {
        if bar.is_bull() {
            self.colors.bull
        } else if bar.is_bear() {
            self.colors.bear
        } else {
            self.colors.undef
        }
    }
    fn get_bar_info<'a>(
        &self,
        chart: &'a Chart,
    ) -> egui_plot::CoordinatesFormatter<'a> {
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
    fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.heading("Chart widget");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tf, TimeFrame::M1, "1M");
            ui.selectable_value(&mut self.tf, TimeFrame::M10, "10M");
            ui.selectable_value(&mut self.tf, TimeFrame::H1, "1H");
            ui.selectable_value(&mut self.tf, TimeFrame::Day, "D");
            ui.selectable_value(&mut self.tf, TimeFrame::Week, "W");
            ui.selectable_value(&mut self.tf, TimeFrame::Month, "M");
        });
        ui.separator();
    }
    fn show_empty(&self, ui: &mut egui::Ui) {
        Plot::new("chart_plot")
            .show_grid(false)
            .show(ui, |_plot_ui| {});
    }
    fn show_chart(&self, ui: &mut egui::Ui, asset: &mut Asset) {
        let chart = self.get_chart(asset);
        let step = chart.iid().step();
        let last_price = chart.last_price().unwrap();

        Plot::new("chart_plot")
            .show_grid(false)
            .cursor_color(self.colors.cross)
            .coordinates_formatter(Corner::LeftTop, self.get_bar_info(chart))
            .label_formatter(|_name, value| {
                let price = utils::round_price(value.y, step);
                let pct = (price - last_price) / last_price * 100.0;
                format!(" {:.2}%\n {}", pct, price)
            })
            .show(ui, |plot_ui| {
                for bar in chart.bars().iter() {
                    let color = self.get_color(bar);
                    let x0 = bar.ts_nanos as f64;
                    let x1 = x0 + chart.tf().nanos() as f64;
                    let x = (x1 + x0) / 2.0;

                    let open = Line::new("", vec![[x0, bar.o], [x, bar.o]])
                        .color(color);
                    let close = Line::new("", vec![[x, bar.c], [x1, bar.c]])
                        .color(color);
                    let shadow = Line::new("", vec![[x, bar.l], [x, bar.h]])
                        .color(color);

                    plot_ui.line(open);
                    plot_ui.line(shadow);
                    plot_ui.line(close);
                }
            });
    }
}
