/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;

use crate::scanner::scan_table::ScanTable;
use crate::scanner::view::ScanView;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Scanner {
    scan_table: ScanTable,
    #[serde(skip)]
    scan_view: ScanView,
}
impl Scanner {
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

        Scanner::default()
    }
}
impl Default for Scanner {
    fn default() -> Self {
        Self {
            scan_table: ScanTable::new(),
            scan_view: ScanView::default(),
        }
    }
}
impl eframe::App for Scanner {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        ui_top(self, ctx);
        ui_left(self, ctx);
        ui_center(self, ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Called by the frame work to save state before shutdown.
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn ui_top(_app: &mut Scanner, ctx: &egui::Context) {
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
fn ui_left(app: &mut Scanner, ctx: &egui::Context) {
    egui::SidePanel::left("scan_table").show(ctx, |ui| {
        app.scan_table.ui(ctx, ui);
    });
}
fn ui_center(app: &mut Scanner, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let scan_result = app.scan_table.current_result();
        app.scan_view.ui(ui, scan_result);
    });
}
