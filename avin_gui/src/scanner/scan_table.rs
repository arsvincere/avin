/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_file_dialog::FileDialog;

use avin_filter::{FilterResult, FilterResultList};
use avin_utils::CFG;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ScanTable {
    #[serde(skip)]
    scan_results: FilterResultList,
    current_index: usize,
    #[serde(skip)]
    file_dialog: FileDialog,
}
impl ScanTable {
    pub fn new() -> Self {
        ScanTable::default()
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.ui_toolbar(ctx, ui);
        self.ui_table(ui);
    }
    pub fn current_result(&mut self) -> Option<&FilterResult> {
        self.scan_results.get(self.current_index)
    }

    // private
    fn ui_toolbar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {
                self.file_dialog.pick_directory();
            }

            // Update the dialog
            self.file_dialog.update(ctx);

            // Check if the user picked a file.
            if let Some(path) = self.file_dialog.take_picked() {
                self.scan_results = FilterResultList::load_dir(&path).unwrap();
                self.current_index = 0;
            };
        });

        ui.separator();
    }
    fn ui_table(&mut self, ui: &mut egui::Ui) {
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
                    ui.strong("Name");
                });
            })
            .body(|body| {
                body.rows(text_height, self.scan_results.len(), |mut row| {
                    let index = row.index();
                    let scan_result = self.scan_results.get(index).unwrap();
                    if self.current_index == index {
                        row.set_selected(true);
                    }
                    row.col(|ui| {
                        ui.label(scan_result.scan_name());
                    });
                    if row.response().clicked() {
                        self.current_index = row.index();
                    };
                });
            });
    }
}
impl Default for ScanTable {
    fn default() -> Self {
        let path = CFG.dir.scan();
        let file_dialog = FileDialog::new().initial_directory(path);

        Self {
            scan_results: FilterResultList::new(),
            current_index: 0,
            file_dialog,
        }
    }
}
