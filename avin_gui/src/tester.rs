/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;

use super::chart::ChartWidget;
use super::test::TestWidget;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Tester {
    #[serde(skip)]
    test_widget: TestWidget,
    #[serde(skip)]
    chart_widget: ChartWidget,
}
impl Default for Tester {
    fn default() -> Self {
        Self {
            test_widget: TestWidget::new(),
            chart_widget: ChartWidget::default(),
        }
    }
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
impl eframe::App for Tester {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        ui_top(self, ctx);

        ui_left(self, ctx);
        ui_right(self, ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Called by the frame work to save state before shutdown.
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

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
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        app.test_widget.ui(ctx, ui);
    });
}
fn ui_right(app: &mut Tester, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |_ui| {
        let _test = app.test_widget.current_test();
        // app.chart_widget.ui(ui, test);
    });
}
