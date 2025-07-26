/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TestToolbar {
    trades: bool,
    orders: bool,
    operations: bool,
    transactions: bool,
}
impl TestToolbar {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // show trades
            if ui.selectable_label(self.trades, "Trades").clicked() {
                self.trades = !self.trades;
            };
            // show orders
            if ui.selectable_label(self.orders, "Orders").clicked() {
                self.orders = !self.orders;
            };
            // show operations
            if ui.selectable_label(self.operations, "Operations").clicked() {
                self.operations = !self.operations;
            };
            // show transactions
            if ui.selectable_label(self.transactions, "Transact").clicked() {
                self.transactions = !self.transactions;
            };
            ui.separator();
        });
    }
    pub fn is_trades(&self) -> bool {
        self.trades
    }
    pub fn is_orders(&self) -> bool {
        self.orders
    }
    pub fn is_operations(&self) -> bool {
        self.operations
    }
    pub fn is_transactions(&self) -> bool {
        self.transactions
    }
}
impl Default for TestToolbar {
    fn default() -> Self {
        Self {
            trades: true,
            orders: false,
            operations: false,
            transactions: false,
        }
    }
}
