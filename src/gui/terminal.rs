use eframe::egui;
use egui::Align2;
use egui::Color32;
use egui::FontId;
use egui::Mesh;
use egui::Rect;
use egui::Sense;
use egui::Stroke;
use egui::vec2;

use super::asset::AssetWidget;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Terminal {
    #[serde(skip)]
    asset_widet: AssetWidget,
    #[serde(skip)]
    label: String,
}
impl Default for Terminal {
    fn default() -> Self {
        Self {
            // gui
            asset_widet: AssetWidget::new(),
            // state
            label: "Hello World!".to_owned(),
        }
    }
}
impl Terminal {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY)
                .unwrap_or_default();
        }

        // Default::default()
        Terminal::default()
    }
}
impl eframe::App for Terminal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Called each time the UI needs repainting, which may be many times per second.
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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
            self.asset_widet.update(ctx, ui);
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Right...");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.separator();

            let size = vec2(512.0, 512.0);
            let (response, painter) =
                ui.allocate_painter(size, Sense::hover());
            let rect = response.rect;

            let mut top_half = rect;
            top_half.set_bottom(top_half.center().y);
            painter.rect_filled(top_half, 0.0, Color32::BLACK);
            paint_fine_lines_and_text(&painter, top_half, Color32::WHITE);
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

fn paint_fine_lines_and_text(
    painter: &egui::Painter,
    mut rect: Rect,
    color: Color32,
) {
    {
        let mut y = 0.0;
        for opacity in [1.00, 0.50, 0.25, 0.10, 0.05, 0.02, 0.01, 0.00] {
            painter.text(
                rect.center_top() + vec2(0.0, y),
                Align2::LEFT_TOP,
                format!("{:.0}% white", 100.0 * opacity),
                FontId::proportional(14.0),
                Color32::WHITE.gamma_multiply(opacity),
            );
            painter.text(
                rect.center_top() + vec2(80.0, y),
                Align2::LEFT_TOP,
                format!("{:.0}% gray", 100.0 * opacity),
                FontId::proportional(14.0),
                Color32::GRAY.gamma_multiply(opacity),
            );
            painter.text(
                rect.center_top() + vec2(160.0, y),
                Align2::LEFT_TOP,
                format!("{:.0}% black", 100.0 * opacity),
                FontId::proportional(14.0),
                Color32::BLACK.gamma_multiply(opacity),
            );
            y += 20.0;
        }

        for font_size in [6.0, 7.0, 8.0, 9.0, 10.0, 12.0, 14.0] {
            painter.text(
                rect.center_top() + vec2(0.0, y),
                Align2::LEFT_TOP,
                format!(
                    "{font_size}px - The quick brown fox jumps over the lazy dog and runs away."
                ),
                FontId::proportional(font_size),
                color,
            );
            y += font_size + 1.0;
        }
    }

    rect.max.x = rect.center().x;

    rect = rect.shrink(16.0);
    for width in [0.05, 0.1, 0.25, 0.5, 1.0, 2.0, 4.0] {
        painter.text(
            rect.left_top(),
            Align2::CENTER_CENTER,
            width.to_string(),
            FontId::monospace(12.0),
            color,
        );

        painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
            [
                rect.left_top() + vec2(16.0, 0.0),
                rect.right_top(),
                rect.right_center(),
                rect.right_bottom(),
            ],
            false,
            Color32::TRANSPARENT,
            Stroke::new(width, color),
        ));

        rect.min.y += 24.0;
        rect.max.x -= 24.0;
    }

    rect.min.y += 16.0;
    painter.text(
        rect.left_top(),
        Align2::LEFT_CENTER,
        "transparent --> opaque",
        FontId::monospace(10.0),
        color,
    );
    rect.min.y += 12.0;
    let mut mesh = Mesh::default();
    mesh.colored_vertex(rect.left_bottom(), Color32::TRANSPARENT);
    mesh.colored_vertex(rect.left_top(), Color32::TRANSPARENT);
    mesh.colored_vertex(rect.right_bottom(), color);
    mesh.colored_vertex(rect.right_top(), color);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(1, 2, 3);
    painter.add(mesh);
}
