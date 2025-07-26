/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use egui_extras::{Column, TableBuilder};

use avin_core::Summary;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SummaryTable {}
impl SummaryTable {
    pub fn ui(&mut self, ui: &mut egui::Ui, summary: &Summary) {
        let available_height = 100.0;
        let mut table = TableBuilder::new(ui)
            .striped(false) // чередующаяся подсветка строк
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);
        table = table.sense(egui::Sense::click());

        let text_height = 24.0;
        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("name");
                });
                header.col(|ui| {
                    ui.strong("profit");
                });
                header.col(|ui| {
                    ui.strong("%");
                });
                header.col(|ui| {
                    ui.strong("trades");
                });
                header.col(|ui| {
                    ui.strong("win");
                });
                header.col(|ui| {
                    ui.strong("loss");
                });
                header.col(|ui| {
                    ui.strong("ratio");
                });
                header.col(|ui| {
                    ui.strong("avg");
                });
                header.col(|ui| {
                    ui.strong("w-seq");
                });
                header.col(|ui| {
                    ui.strong("l-seq");
                });
                header.col(|ui| {
                    ui.strong("gross profit");
                });
                header.col(|ui| {
                    ui.strong("gross loss");
                });
            })
            .body(|body| {
                body.rows(text_height, 1, |mut row| {
                    row.col(|ui| {
                        ui.label(summary.name.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.profit.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.percent_profitable.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.total_trades.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.win_trades.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.loss_trades.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.ratio.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.average_trade.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.win_seq.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.loss_seq.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.gross_profit.to_string());
                    });
                    row.col(|ui| {
                        ui.label(summary.gross_loss.to_string());
                    });
                });
            });
    }
}
