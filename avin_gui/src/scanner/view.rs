/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use eframe::egui::Key;
use egui_plot::Corner;
use egui_plot::Plot;

use avin_core::{
    Asset, Chart, ExtremumIndicator, Footprint,
    Term::{T1, T2, T3, T4, T5},
};
use avin_strategy::ScannerResult;
use avin_utils::CFG;
use egui_plot::PlotUi;

use crate::chart_widget::ChartToolbar;
use crate::draw::{ChartDraw, FootprintDraw, ScanDraw};
use crate::theme::Theme;

pub struct ScanView {
    chart_toolbar: ChartToolbar,
    scale_x: bool,
    scale_y: bool,
    theme: Theme,
}
impl Default for ScanView {
    fn default() -> Self {
        Self {
            chart_toolbar: ChartToolbar::default(),
            scale_x: true,
            scale_y: false,
            theme: Theme::default(),
        }
    }
}
impl ScanView {
    // pub
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        scan_result: Option<&ScannerResult>,
    ) {
        self.chart_toolbar.ui(ui);

        match scan_result {
            Some(scan_result) => self.show_scan(ui, scan_result),
            None => self.show_empty(ui),
        };
    }

    // private
    fn show_empty(&self, ui: &mut egui::Ui) {
        Plot::new("chart_plot")
            .show_grid(false)
            .show(ui, |_plot_ui| {});
    }
    fn show_scan(&mut self, ui: &mut egui::Ui, scan_result: &ScannerResult) {
        // get asset
        let mut asset = Asset::new(scan_result.iid_name()).unwrap();
        let tf = self.chart_toolbar.tf();

        // load charts
        match asset.chart(tf) {
            Some(_) => (),
            None => {
                let b = scan_result.begin();
                let e = scan_result.end();
                asset.load_chart_period(tf, &b, &e).unwrap();

                let chart = asset.chart_mut(tf).unwrap();
                ExtremumIndicator::init(chart);
            }
        };

        // scaling
        self.scale(ui);

        // drawing
        ui.vertical(|ui| {
            self.build_center_plot(ui, &asset, scan_result);
            self.build_bottom_plot(ui, &asset, scan_result);
        });

        // view trades
        // view orders
        // view operations
        // view transactions
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
        scan_result: &ScannerResult,
    ) -> egui_plot::PlotResponse<()> {
        let chart = asset.chart(self.chart_toolbar.tf()).unwrap();

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
            .show(ui, |plot_ui| self.draw_center(plot_ui, chart, scan_result))
    }
    fn build_bottom_plot(
        &self,
        ui: &mut egui::Ui,
        asset: &Asset,
        _scan_result: &ScannerResult,
    ) -> egui_plot::PlotResponse<()> {
        let footprint = asset.footprint(self.chart_toolbar.tf());

        Plot::new("bottom_plot")
            .link_axis("link_group", [true, false])
            .link_cursor("link_group", [true, false])
            .height(CFG.gui.chart.bottom_pane_height)
            .show_grid(false)
            .show_axes([false, false])
            .allow_zoom([self.scale_x, self.scale_y])
            .cursor_color(self.theme.cross)
            .show(ui, |plot_ui| self.draw_bottom(plot_ui, footprint))
    }
    fn draw_center(
        &self,
        plot_ui: &mut PlotUi,
        chart: &Chart,
        scan_result: &ScannerResult,
    ) {
        // draw bars
        if self.chart_toolbar.is_bars() {
            chart.draw_bars(plot_ui, &self.theme);
        }

        // draw trends
        if self.chart_toolbar.is_t1() {
            chart.draw_trends(plot_ui, &self.theme, T1);
            chart.draw_posterior_1(plot_ui, &self.theme, T1);
            chart.draw_posterior_0(plot_ui, &self.theme, T1);
        }
        if self.chart_toolbar.is_t2() {
            chart.draw_trends(plot_ui, &self.theme, T2);
            chart.draw_posterior_1(plot_ui, &self.theme, T2);
            chart.draw_posterior_0(plot_ui, &self.theme, T2);
        }
        if self.chart_toolbar.is_t3() {
            chart.draw_trends(plot_ui, &self.theme, T3);
            chart.draw_posterior_1(plot_ui, &self.theme, T3);
            chart.draw_posterior_0(plot_ui, &self.theme, T3);
        }
        if self.chart_toolbar.is_t4() {
            chart.draw_trends(plot_ui, &self.theme, T4);
            chart.draw_posterior_1(plot_ui, &self.theme, T4);
            chart.draw_posterior_0(plot_ui, &self.theme, T4);
        }
        if self.chart_toolbar.is_t5() {
            chart.draw_trends(plot_ui, &self.theme, T5);
            chart.draw_posterior_1(plot_ui, &self.theme, T5);
            chart.draw_posterior_0(plot_ui, &self.theme, T5);
        }

        // draw quantum
        if self.chart_toolbar.is_quantum() {
            // draw_quantum(plot_ui, &self.theme, footprint);
        }

        // draw scan points
        scan_result.draw_points(plot_ui, &self.theme, self.chart_toolbar.tf())
    }
    fn draw_bottom(
        &self,
        plot_ui: &mut PlotUi,
        footprint: Option<&Footprint>,
    ) {
        if let Some(f) = footprint {
            f.draw_hist(plot_ui, &self.theme);
        }
    }
}
