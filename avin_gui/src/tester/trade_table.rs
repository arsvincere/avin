/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::Trade;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_file_dialog::FileDialog;

use avin_tester::Test;
use avin_utils::CFG;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TradeTable {
    current_index: usize,
    #[serde(skip)]
    file_dialog: FileDialog,
}
impl TradeTable {
    pub fn ui(&mut self, ui: &mut egui::Ui, test: &Test) {
        self.ui_table(ui, test);
    }

    // private
    fn ui_table(&mut self, ui: &mut egui::Ui, test: &Test) {
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
                    ui.strong("DT");
                });
            })
            .body(|body| {
                body.rows(text_height, test.trade_list.len(), |mut row| {
                    let index = row.index();
                    let trade = test.trade_list.trades().get(index).unwrap();
                    let trade = match trade {
                        Trade::Closed(t) => t,
                        _ => unreachable!(),
                    };
                    if self.current_index == index {
                        row.set_selected(true);
                    }

                    row.col(|ui| {
                        ui.label(trade.open_dt().to_string());
                    });
                    if row.response().clicked() {
                        self.current_index = row.index();
                    };
                });
            });
    }
}
impl Default for TradeTable {
    fn default() -> Self {
        let path = CFG.dir.test();
        let file_dialog = FileDialog::new().initial_directory(path);

        Self {
            current_index: 0,
            file_dialog,
        }
    }
}
