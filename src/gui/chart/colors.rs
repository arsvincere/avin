use eframe::egui::Color32;

use crate::conf;

pub struct Colors {
    pub cross: Color32,
    pub bear: Color32,
    pub bull: Color32,
    pub undef: Color32,
}
impl Default for Colors {
    fn default() -> Self {
        Self {
            cross: Color32::from_hex(conf::CROSS).unwrap(),
            bear: Color32::from_hex(conf::BEAR).unwrap(),
            bull: Color32::from_hex(conf::BULL).unwrap(),
            undef: Color32::from_hex(conf::UNDEFINE).unwrap(),
        }
    }
}
