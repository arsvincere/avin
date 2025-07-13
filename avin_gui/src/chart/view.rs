/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use eframe::egui::Key;
use egui_plot::{Corner, Plot, PlotUi};

use avin_core::{
    Asset, Chart, Footprint,
    Term::{T1, T2, T3, T4, T5},
};
use avin_utils::CFG;

use crate::chart::draw::{ChartDraw, FootprintDraw};
use crate::theme::Theme;

use super::toolbar::ChartToolbar;

pub struct ChartView {
    scale_x: bool,
    scale_y: bool,
    theme: Theme,
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
            chart.draw_posterior_1(plot_ui, &self.theme, T1);
            chart.draw_posterior_0(plot_ui, &self.theme, T1);
        }
        if cfg.is_t2() {
            chart.draw_trends(plot_ui, &self.theme, T2);
            chart.draw_posterior_1(plot_ui, &self.theme, T2);
            chart.draw_posterior_0(plot_ui, &self.theme, T2);
        }
        if cfg.is_t3() {
            chart.draw_trends(plot_ui, &self.theme, T3);
            chart.draw_posterior_1(plot_ui, &self.theme, T3);
            chart.draw_posterior_0(plot_ui, &self.theme, T3);
        }
        if cfg.is_t4() {
            chart.draw_trends(plot_ui, &self.theme, T4);
            chart.draw_posterior_1(plot_ui, &self.theme, T4);
            chart.draw_posterior_0(plot_ui, &self.theme, T4);
        }
        if cfg.is_t5() {
            chart.draw_trends(plot_ui, &self.theme, T5);
            chart.draw_posterior_1(plot_ui, &self.theme, T5);
            chart.draw_posterior_0(plot_ui, &self.theme, T5);
        }

        // draw quantum
        if cfg.is_quantum() {
            // draw_quantum(plot_ui, &self.theme, footprint);
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
impl Default for ChartView {
    fn default() -> Self {
        Self {
            scale_x: true,
            scale_y: true,
            theme: Theme::default(),
        }
    }
}
