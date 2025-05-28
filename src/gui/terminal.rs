/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;

use super::asset::AssetWidget;
use super::chart::ChartWidget;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Terminal {
    #[serde(skip)]
    asset_widget: AssetWidget,
    #[serde(skip)]
    chart_widget: ChartWidget,

    #[serde(skip)]
    label: String,
}
impl Default for Terminal {
    fn default() -> Self {
        Self {
            asset_widget: AssetWidget::new(),
            chart_widget: ChartWidget::new(),

            // delete
            label: "Hello World!".to_owned(),
        }
    }
}
impl Terminal {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Called once before the first frame.
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY)
                .unwrap_or_default();
        }

        Terminal::default()
    }
}
impl eframe::App for Terminal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(
                                egui::ViewportCommand::Close,
                            );
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            self.asset_widget.ui(ctx, ui);
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Right...");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let asset = self.asset_widget.current_asset();
            self.chart_widget.ui(ui, asset);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.heading("Console");
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Called by the frame work to save state before shutdown.
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
