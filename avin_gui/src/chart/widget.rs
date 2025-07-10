/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use egui_plot::Plot;

use avin_core::{Asset, ExtremumIndicator, TimeFrame};

use super::toolbar::ChartToolbar;
use super::view::ChartView;

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
        ensure_chart(asset, self.toolbar.tf());
        ensure_footprint(asset, self.toolbar.tf());

        self.view.draw(ui, asset, &self.toolbar);
    }
}

fn ensure_chart(asset: &mut Asset, tf: &TimeFrame) {
    match asset.chart(tf) {
        Some(_) => (),
        None => {
            asset.load_chart(tf).unwrap();
            let chart = asset.chart_mut(tf).unwrap();
            ExtremumIndicator::init(chart);
        }
    }
}
fn ensure_footprint(asset: &mut Asset, tf: &TimeFrame) {
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
}
