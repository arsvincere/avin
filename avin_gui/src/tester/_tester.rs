/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::Summary;
use eframe::egui;

use crate::tester::summary_table::SummaryTable;
use crate::tester::test_table::TestTable;
use crate::tester::trade_table::TradeTable;
use crate::tester::view::TestView;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Tester {
    test_table: TestTable,
    #[serde(skip)]
    test_view: TestView,
    trade_table: TradeTable,
    summary_table: SummaryTable,
}
impl Tester {
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

        Tester::default()
    }
}
impl Default for Tester {
    fn default() -> Self {
        Self {
            test_table: TestTable::new(),
            test_view: TestView::default(),
            trade_table: TradeTable::default(),
            summary_table: SummaryTable::default(),
        }
    }
}
impl eframe::App for Tester {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        ui_top(self, ctx);
        ui_left(self, ctx);
        ui_center(self, ctx);
        ui_right(self, ctx);
        ui_bottom(self, ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Called by the frame work to save state before shutdown.
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

#[allow(deprecated)]
fn ui_top(_app: &mut Tester, ctx: &egui::Context) {
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
            ui.label("Active:");
        });
    });
}
fn ui_left(app: &mut Tester, ctx: &egui::Context) {
    egui::SidePanel::left("test_table").show(ctx, |ui| {
        app.test_table.ui(ctx, ui);
    });
}
fn ui_center(app: &mut Tester, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let test = app.test_table.current_test();
        app.test_view.ui(ui, test);
    });
}
fn ui_right(app: &mut Tester, ctx: &egui::Context) {
    egui::SidePanel::right("trade_table").show(ctx, |ui| {
        if let Some(test) = app.test_table.current_test() {
            app.trade_table.ui(ui, test);
        }
    });
}
fn ui_bottom(app: &mut Tester, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("summary_table").show(ctx, |ui| {
        if let Some(test) = app.test_table.current_test() {
            let summary = Summary::new(&test.trade_list);
            app.summary_table.ui(ui, &summary);
        }
        ui.label("");
    });
}
