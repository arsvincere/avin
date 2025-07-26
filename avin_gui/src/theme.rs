/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use eframe::egui::Color32;

use avin_utils::CFG;

pub struct Theme {
    pub red: Color32,
    pub orange: Color32,
    pub yellow: Color32,
    pub green: Color32,
    pub cyan: Color32,
    pub blue: Color32,
    pub violet: Color32,
    pub white: Color32,
    pub grey: Color32,
    pub black: Color32,

    pub cross: Color32,
    pub bear: Color32,
    pub bull: Color32,
    pub undef: Color32,
    pub t1: Color32,
    pub t2: Color32,
    pub t3: Color32,
    pub t4: Color32,
    pub t5: Color32,

    pub trade_open: Color32,
    pub trade_stop: Color32,
    pub trade_take: Color32,
}
impl Default for Theme {
    fn default() -> Self {
        let c = &CFG.gui.color;

        Self {
            red: Color32::from_hex(&c.red).unwrap(),
            orange: Color32::from_hex(&c.orange).unwrap(),
            yellow: Color32::from_hex(&c.yellow).unwrap(),
            green: Color32::from_hex(&c.green).unwrap(),
            cyan: Color32::from_hex(&c.cyan).unwrap(),
            blue: Color32::from_hex(&c.blue).unwrap(),
            violet: Color32::from_hex(&c.violet).unwrap(),
            white: Color32::from_hex(&c.white).unwrap(),
            grey: Color32::from_hex(&c.grey).unwrap(),
            black: Color32::from_hex(&c.black).unwrap(),

            cross: Color32::from_hex(&c.cross).unwrap(),
            bear: Color32::from_hex(&c.bear)
                .unwrap()
                .gamma_multiply(c.bear_opacity),
            bull: Color32::from_hex(&c.bull)
                .unwrap()
                .gamma_multiply(c.bull_opacity),
            undef: Color32::from_hex(&c.nobody)
                .unwrap()
                .gamma_multiply(c.nobody_opacity),
            t1: Color32::from_hex(&c.trend_t1)
                .unwrap()
                .gamma_multiply(c.trend_t1_opacity),
            t2: Color32::from_hex(&c.trend_t2)
                .unwrap()
                .gamma_multiply(c.trend_t2_opacity),
            t3: Color32::from_hex(&c.trend_t3)
                .unwrap()
                .gamma_multiply(c.trend_t3_opacity),
            t4: Color32::from_hex(&c.trend_t4)
                .unwrap()
                .gamma_multiply(c.trend_t4_opacity),
            t5: Color32::from_hex(&c.trend_t5)
                .unwrap()
                .gamma_multiply(c.trend_t5_opacity),

            trade_open: Color32::from_hex(&c.trade_open).unwrap(),
            trade_stop: Color32::from_hex(&c.trade_stop).unwrap(),
            trade_take: Color32::from_hex(&c.trade_take).unwrap(),
        }
    }
}
