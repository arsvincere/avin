/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::Asset;
use avin_simulator::Simulator;
use avin_utils as utils;
use eframe::egui;

use crate::chart_widget::ChartWidget;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GuiSimulator {
    #[serde(skip)]
    simulator: Simulator,
    step: usize,

    chart_widget: ChartWidget,
}
impl Default for GuiSimulator {
    fn default() -> Self {
        let asset = Asset::new("moex_share_sber").unwrap();
        let begin = utils::str_date_to_utc("2024-01-01");
        let end = utils::str_date_to_utc("2025-01-01");
        Self {
            simulator: Simulator::new(asset.iid(), begin, end),
            step: 1,

            chart_widget: ChartWidget::default(),
        }
    }
}
impl GuiSimulator {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Called once before the first frame.
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY)
                .unwrap_or_default();
        }

        GuiSimulator::default()
    }
}
impl eframe::App for GuiSimulator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui_top(self, ctx);
        ui_center(self, ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Called by the frame work to save state before shutdown.
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn ui_top(app: &mut GuiSimulator, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.add_space(16.0);
            egui::widgets::global_theme_preference_buttons(ui);
            ui.separator();

            ui.label("Step size: ");
            ui.selectable_value(&mut app.step, 1, "1M");
            ui.selectable_value(&mut app.step, 10, "10M");
            ui.selectable_value(&mut app.step, 60, "1H");
            ui.selectable_value(&mut app.step, 24 * 60, "D");
            ui.selectable_value(&mut app.step, 7 * 24 * 60, "W");
            ui.separator();

            if ui.button("Step ->").clicked() {
                app.simulator.step(app.step);
            }
            if ui.button("Restart").clicked() {
                app.simulator.restart();
            }
            ui.separator();
        });
    });
}
fn ui_center(app: &mut GuiSimulator, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let asset = app.simulator.asset_mut();
        app.chart_widget.ui(ui, Some(asset));
    });
}
