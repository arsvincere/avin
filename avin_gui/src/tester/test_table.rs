/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_file_dialog::FileDialog;

use avin_tester::{Test, TestList};
use avin_utils::CFG;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TestTable {
    #[serde(skip)]
    test_list: TestList,
    current_index: usize,
    #[serde(skip)]
    file_dialog: FileDialog,
}
impl TestTable {
    pub fn new() -> Self {
        TestTable::default()
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.ui_toolbar(ctx, ui);
        self.ui_table(ui);
    }
    pub fn current_test(&mut self) -> Option<&Test> {
        self.test_list.get(self.current_index)
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
                self.test_list = TestList::load_dir(&path).unwrap();
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
                body.rows(text_height, self.test_list.len(), |mut row| {
                    let index = row.index();
                    let test = self.test_list.get(index).unwrap();
                    if self.current_index == index {
                        row.set_selected(true);
                    }
                    row.col(|ui| {
                        ui.label(test.name());
                    });
                    if row.response().clicked() {
                        self.current_index = row.index();
                    };
                });
            });
    }
}
impl Default for TestTable {
    fn default() -> Self {
        let path = CFG.dir.test();
        let file_dialog = FileDialog::new().initial_directory(path);

        Self {
            test_list: TestList::new(),
            current_index: 0,
            file_dialog,
        }
    }
}
