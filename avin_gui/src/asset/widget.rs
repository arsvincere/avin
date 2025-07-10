/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_file_dialog::FileDialog;

use avin_core::{
    Action, Asset, AssetList, DataAction, Event, ExtremumIndicator, TimeFrame,
};
use avin_utils::{CFG, Cmd};

pub struct AssetWidget {
    asset_list: AssetList,
    current_index: usize,
    file_dialog: FileDialog,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
}
impl AssetWidget {
    pub fn new(
        event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
        action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
    ) -> Self {
        let mut path = CFG.dir.asset();
        path.push(&CFG.core.default_asset_list);

        let asset_list = if Cmd::is_exist(&path) {
            AssetList::load(&path).unwrap()
        } else {
            AssetList::new("Load")
        };

        let path = CFG.dir.asset();
        let file_dialog = FileDialog::new().initial_directory(path);

        Self {
            asset_list,
            current_index: 0,
            file_dialog,
            event_rx,
            action_tx,
        }

        // TODO: save/load state
        // AssetWidget::default()
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.ui_toolbar(ctx, ui);
        self.ui_table(ui);
        self.receive_market_data();
    }
    pub fn current_asset(&mut self) -> Option<&mut Asset> {
        self.asset_list.get_mut(self.current_index)
    }

    // private
    fn ui_toolbar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
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
                    if row.response().double_clicked() {
                        self.current_index = row.index();
                        self.subscribe_market_data()
                    }
                });
            });
    }

    fn load_charts(&mut self) {
        let asset = self.asset_list.get_mut(self.current_index).unwrap();

        for tf in TimeFrame::all() {
            match asset.chart(&tf).is_some() {
                true => (),
                false => {
                    log::debug!("Asset widget loading {asset} {tf}");
                    asset.load_chart(&tf).unwrap();
                    let chart = asset.chart_mut(&tf).unwrap();
                    ExtremumIndicator::init(chart);
                }
            };
        }
    }
    fn subscribe_market_data(&mut self) {
        self.load_charts();

        let asset = self.asset_list.get(self.current_index).unwrap();
        let iid = asset.iid();

        let mut market_data = Vec::new();
        for tf in TimeFrame::all() {
            let md = tf.market_data();
            market_data.push(md);
        }

        let action =
            Action::Subscribe(DataAction::new(iid.clone(), market_data));
        log::debug!("Asset widget send {action}");
        match self.action_tx.send(action) {
            Ok(_) => (),
            Err(e) => log::error!("{e}"),
        };
    }
    fn receive_market_data(&mut self) {
        while let Ok(event) = self.event_rx.try_recv() {
            log::debug!("Asset widget receive {event}");
            let asset = self.asset_list.get_mut(self.current_index).unwrap();

            match event {
                Event::Bar(e) => asset.bar_event(e),
                Event::Tic(e) => todo!("{:?}", e),
                Event::Order(e) => todo!("{:?}", e),
            }
        }
    }
}
