/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use egui_plot::Plot;

use avin_core::Asset;
use avin_core::ExtremumIndicator;
use avin_tester::Test;

use crate::chart::ChartToolbar;
use crate::chart::ChartView;
use crate::tester::toolbar::TestToolbar;

#[derive(Default)]
pub struct TestView {
    chart_toolbar: ChartToolbar,
    test_toolbar: TestToolbar,
    chart_view: ChartView,
}
impl TestView {
    // pub
    pub fn ui(&mut self, ui: &mut egui::Ui, test: Option<&Test>) {
        self.chart_toolbar.ui(ui);
        self.test_toolbar.ui(ui);

        match test {
            Some(test) => self.show_test(ui, test),
            None => self.show_empty(ui),
        };
    }

    // private
    fn show_empty(&self, ui: &mut egui::Ui) {
        Plot::new("chart_plot")
            .show_grid(false)
            .show(ui, |_plot_ui| {});
    }
    fn show_test(&mut self, ui: &mut egui::Ui, test: &Test) {
        // get asset
        let mut asset = Asset::from_iid(test.iid.clone());
        let tf = self.chart_toolbar.tf();

        // load charts
        match asset.chart(tf) {
            Some(_) => (),
            None => {
                let b = test.begin();
                let e = test.end();
                asset.load_chart_period(tf, &b, &e).unwrap();

                let chart = asset.chart_mut(tf).unwrap();
                ExtremumIndicator::init(chart);
            }
        };

        // view chart
        self.chart_view.draw(ui, &mut asset, &self.chart_toolbar);

        // view trades
        // view orders
        // view operations
        // view transactions
    }
}
