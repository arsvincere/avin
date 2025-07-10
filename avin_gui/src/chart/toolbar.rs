/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::TimeFrame;
use eframe::egui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ChartToolbar {
    #[serde(skip)] // TODO: del it in 0.2.11
    tf1: TimeFrame,
    #[serde(skip)] // TODO: del it in 0.2.11
    tf2: TimeFrame,
    bars: bool,
    quantum: bool,
    t1: bool,
    t2: bool,
    t3: bool,
    t4: bool,
    t5: bool,
}
impl ChartToolbar {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // timeframes
            ui.selectable_value(&mut self.tf1, TimeFrame::M1, "1M");
            ui.selectable_value(&mut self.tf1, TimeFrame::M10, "10M");
            ui.selectable_value(&mut self.tf1, TimeFrame::H1, "1H");
            ui.selectable_value(&mut self.tf1, TimeFrame::Day, "D");
            ui.selectable_value(&mut self.tf1, TimeFrame::Week, "W");
            ui.selectable_value(&mut self.tf1, TimeFrame::Month, "M");
            ui.separator();

            // background timeframes
            ui.selectable_value(&mut self.tf2, TimeFrame::H1, "1H");
            ui.selectable_value(&mut self.tf2, TimeFrame::Day, "D");
            ui.selectable_value(&mut self.tf2, TimeFrame::Week, "W");
            ui.selectable_value(&mut self.tf2, TimeFrame::Month, "M");
            ui.separator();

            // show bars
            if ui.selectable_label(self.bars, "Bar").clicked() {
                self.bars = !self.bars;
            };
            // show quantum
            if ui.selectable_label(self.quantum, "Qnt").clicked() {
                self.quantum = !self.quantum;
            };
            ui.separator();

            // show trends
            if ui.selectable_label(self.t1, "T1").clicked() {
                self.t1 = !self.t1
            };
            if ui.selectable_label(self.t2, "T2").clicked() {
                self.t2 = !self.t2;
            };
            if ui.selectable_label(self.t3, "T3").clicked() {
                self.t3 = !self.t3;
            };
            if ui.selectable_label(self.t4, "T4").clicked() {
                self.t4 = !self.t4;
            };
            if ui.selectable_label(self.t5, "T5").clicked() {
                self.t5 = !self.t5;
            };
            ui.separator();

            // let img = egui::Image::new(egui::include_image!(
            //     "../../../res/icon/btn/bar.svg"
            // ));
            // let btn = egui::Button::image(img);
            // ui.add_sized(BTN_SIZE, btn);
            // ui.separator();
        });
    }

    #[inline]
    pub fn tf(&self) -> &TimeFrame {
        &self.tf1
    }
    #[inline]
    #[allow(dead_code)]
    pub fn bg_tf(&self) -> &TimeFrame {
        &self.tf2
    }
    #[inline]
    pub fn is_bars(&self) -> bool {
        self.bars
    }
    #[inline]
    pub fn is_quantum(&self) -> bool {
        self.quantum
    }
    #[inline]
    pub fn is_t1(&self) -> bool {
        self.t1
    }
    #[inline]
    pub fn is_t2(&self) -> bool {
        self.t2
    }
    #[inline]
    pub fn is_t3(&self) -> bool {
        self.t3
    }
    #[inline]
    pub fn is_t4(&self) -> bool {
        self.t4
    }
    #[inline]
    pub fn is_t5(&self) -> bool {
        self.t5
    }
}
impl Default for ChartToolbar {
    fn default() -> Self {
        Self {
            tf1: TimeFrame::Day,
            tf2: TimeFrame::Week,
            bars: true,
            quantum: true,
            t1: false,
            t2: false,
            t3: false,
            t4: false,
            t5: false,
        }
    }
}
