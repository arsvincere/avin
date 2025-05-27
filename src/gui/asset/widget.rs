use std::path::PathBuf;

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_file_dialog::FileDialog;

use crate::ASSET_DIR;
use crate::Asset;
use crate::AssetList;

pub struct AssetWidget {
    asset_list: AssetList,
    current_index: usize,
    file_dialog: FileDialog,
}
impl Default for AssetWidget {
    fn default() -> Self {
        let mut path = PathBuf::from(&ASSET_DIR);
        path.push("xxx.csv");
        let asset_list = AssetList::load(&path).unwrap();

        let path = PathBuf::from(&ASSET_DIR);
        let file_dialog = FileDialog::new().initial_directory(path);

        Self {
            asset_list,
            current_index: 0,
            file_dialog,
        }
    }
}
impl AssetWidget {
    pub fn new() -> Self {
        // TODO: save/load state
        AssetWidget::default()
    }
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Asset list widget");
        ui.horizontal(|ui| {
            if ui.button(self.asset_list.name()).clicked() {
                self.file_dialog.pick_file();
            }
            let _ = ui.button("...");

            // Update the dialog
            self.file_dialog.update(ctx);

            // Check if the user picked a file.
            if let Some(path) = self.file_dialog.take_picked() {
                self.asset_list = AssetList::load(&path).unwrap();
                self.current_index = 0;
            };
        });

        ui.separator();

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(false) // чередующаяся подсветка строк
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        table = table.sense(egui::Sense::click());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Ticker");
                });
            })
            .body(|body| {
                body.rows(text_height, self.asset_list.len(), |mut row| {
                    let index = row.index();
                    let asset = self.asset_list.get(index).unwrap();
                    if self.current_index == index {
                        row.set_selected(true);
                    }
                    row.col(|ui| {
                        ui.label(asset.ticker());
                    });
                    if row.response().clicked() {
                        self.current_index = row.index();
                    };
                });
            });
    }
    pub fn current_asset(&mut self) -> Option<&mut Asset> {
        self.asset_list.get_mut(self.current_index)
    }
}
