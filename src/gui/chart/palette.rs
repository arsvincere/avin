/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui::Color32;

use crate::{
    BEAR_OPACITY, BULL_OPACITY, TREND_T1_OPACITY, TREND_T2_OPACITY,
    TREND_T3_OPACITY, TREND_T4_OPACITY, TREND_T5_OPACITY, UNDEFINE_OPACITY,
    conf,
};

pub struct Palette {
    pub cross: Color32,
    pub bear: Color32,
    pub bull: Color32,
    pub undef: Color32,
    pub t1: Color32,
    pub t2: Color32,
    pub t3: Color32,
    pub t4: Color32,
    pub t5: Color32,
}
impl Default for Palette {
    fn default() -> Self {
        Self {
            cross: Color32::from_hex(conf::CROSS).unwrap(),
            bear: Color32::from_hex(conf::BEAR)
                .unwrap()
                .gamma_multiply(BEAR_OPACITY),
            bull: Color32::from_hex(conf::BULL)
                .unwrap()
                .gamma_multiply(BULL_OPACITY),
            undef: Color32::from_hex(conf::UNDEFINE)
                .unwrap()
                .gamma_multiply(UNDEFINE_OPACITY),
            t1: Color32::from_hex(conf::TREND_T1)
                .unwrap()
                .gamma_multiply(TREND_T1_OPACITY),
            t2: Color32::from_hex(conf::TREND_T2)
                .unwrap()
                .gamma_multiply(TREND_T2_OPACITY),
            t3: Color32::from_hex(conf::TREND_T3)
                .unwrap()
                .gamma_multiply(TREND_T3_OPACITY),
            t4: Color32::from_hex(conf::TREND_T4)
                .unwrap()
                .gamma_multiply(TREND_T4_OPACITY),
            t5: Color32::from_hex(conf::TREND_T5)
                .unwrap()
                .gamma_multiply(TREND_T5_OPACITY),
        }
    }
}
