/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use eframe::egui::Key;
use egui_plot::{Corner, Plot, PlotUi};

use avin_analyse::TrendAnalytic;
use avin_core::{
    Asset, Chart, ExtremumIndicator, Footprint,
    Term::{T1, T2, T3, T4, T5},
    TimeFrame,
};
use avin_utils::CFG;

use crate::draw::{ChartDraw, FootprintDraw};
use crate::theme::Theme;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ChartWidget {
    toolbar: ChartToolbar,
    #[serde(skip)]
    view: ChartView,
}
impl ChartWidget {
    // pub
    pub fn ui(&mut self, ui: &mut egui::Ui, asset: Option<&mut Asset>) {
        self.toolbar.ui(ui);

        match asset {
            Some(asset) => self.show_chart(ui, asset),
            None => self.show_empty(ui),
        };
    }

    // private
    fn show_empty(&self, ui: &mut egui::Ui) {
        Plot::new("chart_plot")
            .show_grid(false)
            .show(ui, |_plot_ui| {});
    }
    fn show_chart(&mut self, ui: &mut egui::Ui, asset: &mut Asset) {
        let tf = self.toolbar.tf();

        // try load chart
        match asset.chart(tf) {
            Some(_) => (),
            None => {
                asset.load_chart(tf).unwrap();
                let chart = asset.chart_mut(tf).unwrap();
                ExtremumIndicator::init(chart);
                TrendAnalytic::init(chart);
            }
        }

        // check tics
        match asset.tics() {
            Some(_) => (),
            None => asset.load_tics().unwrap(),
        };

        // check footprint
        match asset.footprint(tf) {
            Some(_) => (),
            None => asset.build_footprint(tf).unwrap(),
        };

        self.view.draw(ui, asset, &self.toolbar);
    }
}

pub struct ChartView {
    scale_x: bool,
    scale_y: bool,
    theme: Theme,
}
impl Default for ChartView {
    fn default() -> Self {
        Self {
            scale_x: true,
            scale_y: false,
            theme: Theme::default(),
        }
    }
}
impl ChartView {
    pub fn draw(
        &mut self,
        ui: &mut egui::Ui,
        asset: &mut Asset,
        cfg: &ChartToolbar,
    ) {
        self.scale(ui);

        ui.vertical(|ui| {
            self.build_center_plot(ui, asset, cfg);
            self.build_bottom_plot(ui, asset, cfg);
        });
    }
    fn scale(&mut self, ui: &mut egui::Ui) {
        let _ = ui.input(|i| {
            i.events.iter().find_map(|e| match e {
                egui::Event::Key {
                    key,
                    physical_key: _,
                    pressed,
                    repeat: _,
                    modifiers: _,
                } => {
                    if *key == Key::F && *pressed {
                        self.scale_x = false;
                        self.scale_y = true;
                    } else {
                        self.scale_x = true;
                        self.scale_y = false;
                    };
                    Some(())
                }
                _ => None,
            })
        });
    }
    fn build_center_plot(
        &self,
        ui: &mut egui::Ui,
        asset: &Asset,
        cfg: &ChartToolbar,
    ) -> egui_plot::PlotResponse<()> {
        let chart = asset.chart(cfg.tf()).unwrap();

        Plot::new("chart_plot")
            .link_axis("link_group", [true, false])
            .link_cursor("link_group", [true, false])
            .height(ui.available_height() - CFG.gui.chart.bottom_pane_height)
            .show_grid(false)
            .show_axes([false, false])
            .allow_zoom([self.scale_x, self.scale_y])
            .cursor_color(self.theme.cross)
            .coordinates_formatter(Corner::LeftTop, chart.bar_info())
            .label_formatter(|name, value| chart.price_info(name, value))
            .show(ui, |plot_ui| self.draw_center(plot_ui, chart, cfg))
    }
    fn build_bottom_plot(
        &self,
        ui: &mut egui::Ui,
        asset: &Asset,
        cfg: &ChartToolbar,
    ) -> egui_plot::PlotResponse<()> {
        let footprint = asset.footprint(cfg.tf());

        Plot::new("bottom_plot")
            .link_axis("link_group", [true, false])
            .link_cursor("link_group", [true, false])
            .height(CFG.gui.chart.bottom_pane_height)
            .show_grid(false)
            .show_axes([false, false])
            .allow_zoom([self.scale_x, self.scale_y])
            .cursor_color(self.theme.cross)
            .show(ui, |plot_ui| self.draw_bottom(plot_ui, footprint, cfg))
    }
    fn draw_center(
        &self,
        plot_ui: &mut PlotUi,
        chart: &Chart,
        cfg: &ChartToolbar,
    ) {
        // draw bars
        if cfg.is_bars() {
            chart.draw_bars(plot_ui, &self.theme);
        }

        // draw trends
        if cfg.is_t1() {
            chart.draw_trends(plot_ui, &self.theme, T1);
            if cfg.is_p1() {
                chart.draw_posterior_1(plot_ui, &self.theme, T1);
            }
            if cfg.is_p0() {
                chart.draw_posterior_0(plot_ui, &self.theme, T1);
            }
        }
        if cfg.is_t2() {
            chart.draw_trends(plot_ui, &self.theme, T2);
            if cfg.is_p1() {
                chart.draw_posterior_1(plot_ui, &self.theme, T2);
            }
            if cfg.is_p0() {
                chart.draw_posterior_0(plot_ui, &self.theme, T2);
            }
        }
        if cfg.is_t3() {
            chart.draw_trends(plot_ui, &self.theme, T3);
            if cfg.is_p1() {
                chart.draw_posterior_1(plot_ui, &self.theme, T3);
            }
            if cfg.is_p0() {
                chart.draw_posterior_0(plot_ui, &self.theme, T3);
            }
        }
        if cfg.is_t4() {
            chart.draw_trends(plot_ui, &self.theme, T4);
            if cfg.is_p1() {
                chart.draw_posterior_1(plot_ui, &self.theme, T4);
            }
            if cfg.is_p0() {
                chart.draw_posterior_0(plot_ui, &self.theme, T4);
            }
        }
        if cfg.is_t5() {
            chart.draw_trends(plot_ui, &self.theme, T5);
            if cfg.is_p1() {
                chart.draw_posterior_1(plot_ui, &self.theme, T5);
            }
            if cfg.is_p0() {
                chart.draw_posterior_0(plot_ui, &self.theme, T5);
            }
        }
    }
    fn draw_bottom(
        &self,
        plot_ui: &mut PlotUi,
        footprint: Option<&Footprint>,
        _cfg: &ChartToolbar,
    ) {
        if let Some(f) = footprint {
            f.draw_hist(plot_ui, &self.theme);
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ChartToolbar {
    tf1: TimeFrame,
    tf2: TimeFrame,
    bars: bool,
    quantum: bool,
    t1: bool,
    t2: bool,
    t3: bool,
    t4: bool,
    t5: bool,
    p1: bool,
    p0: bool,
}
impl ChartToolbar {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // timeframes
            ui.selectable_value(&mut self.tf1, TimeFrame::M1, "1M");
            ui.selectable_value(&mut self.tf1, TimeFrame::M10, "10M");
            ui.selectable_value(&mut self.tf1, TimeFrame::H1, "1H");
            ui.selectable_value(&mut self.tf1, TimeFrame::Day, "D");
            ui.selectable_value(&mut self.tf1, TimeFrame::Week, "W");
            ui.selectable_value(&mut self.tf1, TimeFrame::Month, "M");
            ui.separator();

            // background timeframes
            ui.selectable_value(&mut self.tf2, TimeFrame::H1, "1H");
            ui.selectable_value(&mut self.tf2, TimeFrame::Day, "D");
            ui.selectable_value(&mut self.tf2, TimeFrame::Week, "W");
            ui.selectable_value(&mut self.tf2, TimeFrame::Month, "M");
            ui.separator();

            // show bars
            if ui.selectable_label(self.bars, "Bar").clicked() {
                self.bars = !self.bars;
            };
            // show quantum
            if ui.selectable_label(self.quantum, "Qnt").clicked() {
                self.quantum = !self.quantum;
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

            if ui.selectable_label(self.p1, "P1").clicked() {
                self.p1 = !self.p1;
            };
            if ui.selectable_label(self.p0, "P0").clicked() {
                self.p0 = !self.p0;
            };
            ui.separator();
        });
    }

    #[inline]
    pub fn tf(&self) -> TimeFrame {
        self.tf1
    }
    #[inline]
    #[allow(dead_code)]
    pub fn bg_tf(&self) -> TimeFrame {
        self.tf2
    }
    #[inline]
    pub fn is_bars(&self) -> bool {
        self.bars
    }
    #[inline]
    pub fn is_quantum(&self) -> bool {
        self.quantum
    }
    #[inline]
    pub fn is_t1(&self) -> bool {
        self.t1
    }
    #[inline]
    pub fn is_t2(&self) -> bool {
        self.t2
    }
    #[inline]
    pub fn is_t3(&self) -> bool {
        self.t3
    }
    #[inline]
    pub fn is_t4(&self) -> bool {
        self.t4
    }
    #[inline]
    pub fn is_t5(&self) -> bool {
        self.t5
    }
    #[inline]
    pub fn is_p1(&self) -> bool {
        self.p1
    }
    #[inline]
    pub fn is_p0(&self) -> bool {
        self.p0
    }
}
impl Default for ChartToolbar {
    fn default() -> Self {
        Self {
            tf1: TimeFrame::Day,
            tf2: TimeFrame::Week,
            bars: true,
            quantum: true,
            t1: false,
            t2: false,
            t3: false,
            t4: false,
            t5: false,
            p1: false,
            p0: false,
        }
    }
}
