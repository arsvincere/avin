/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{NaiveTime, TimeDelta};

// GUI
pub const APP_ID: &str = "com.arsvincere.avin";
//  TODO: del, use egui state
pub const DEFAULT_ASSET_LIST: &str = "xxx.csv";

//  TODO: del, use from config.toml
pub const CHART_BOTTOM: f32 = 200.0;

// Chart
//  TODO: del, use from config.toml
pub const DEFAULT_BARS_COUNT: i32 = 5000;

// TODO: del, use from config.toml
pub const CROSS: &str = "#282727";
pub const BEAR: &str = "#FF5D62";
pub const BULL: &str = "#98BB6C";
pub const UNDEFINE: &str = "#CCCCCC";
pub const BEAR_OPACITY: f32 = 0.5;
pub const BULL_OPACITY: f32 = 0.5;
pub const UNDEFINE_OPACITY: f32 = 0.5;

// TODO: del, use from config.toml
pub const TREND_T1: &str = "#AAAAAA";
pub const TREND_T2: &str = "#658594";
pub const TREND_T3: &str = "#7E9CD8";
pub const TREND_T4: &str = "#957FB8";
pub const TREND_T5: &str = "#DCA561";
pub const TREND_T1_OPACITY: f32 = 0.2;
pub const TREND_T2_OPACITY: f32 = 0.4;
pub const TREND_T3_OPACITY: f32 = 0.6;
pub const TREND_T4_OPACITY: f32 = 0.8;
pub const TREND_T5_OPACITY: f32 = 1.0;

// BG = QtGui.QColor(Color.dragonBlack0)  # #0d0c0c
// BG_FOOTER = QtGui.QColor(Color.dragonBlack5)
// CROSS = QtGui.QColor(Color.dragonBlack6)
//
// BULL_BEHIND = QtGui.QColor("#98BB6C")
// BEAR_BEHIND = QtGui.QColor("#FF5D62")
// UNDEFINE_BEHIND = QtGui.QColor("#FFFFFF")
//
// VOL_BEAR = QtGui.QColor("#33FF5D62")
// VOL_BULL = QtGui.QColor("#3398BB6C")
// VOL_UNDEFINE = QtGui.QColor("#33FFFFFF")
// VOL_FRAME = QtGui.QColor(Color.dragonBlack1)
//
// # Trade
// STOP = QtGui.QColor("#c84053")
// TAKE = QtGui.QColor("#6f894e")
// OPEN = QtGui.QColor("#8a8980")
// TRADE_WIN = QtGui.QColor("#00AA00")
// TRADE_LOSS = QtGui.QColor("#AA0000")
// TRADE_UNDEFINE = QtGui.QColor("#888888")
//
// # Extremum
// VAWE_BEAR = QtGui.QColor("#AA0000")
// VAWE_BULL = QtGui.QColor("#00AA00")
// INSIDE_BG = QtGui.QColor("#000000")
// OUTSIDE_BG = QtGui.QColor("#FFFFFF")
//
// # Posterior
// POSTERIOR_BULL = QtGui.QColor("#98BB6C")
// POSTERIOR_BEAR = QtGui.QColor("#FF5D62")
// POSTERIOR_NOW = QtGui.QColor("#848388")

// Settings
// TODO: del, use from config.toml
pub const DEFAULT_COMMISSION: f64 = 0.0005;

// Dir
// TODO: del, use from config.toml
pub const ASSET_DIR: &str = "/home/alex/avin/usr/asset";
pub const CACHE_DIR: &str = "/home/alex/avin/usr/cache";
pub const DATA_DIR: &str = "/home/alex/avin/usr/data";
pub const TEST_DIR: &str = "/home/alex/avin/usr/test";

// Connect
// TODO: del, use from config.toml
pub const TINKOFF_TOKEN: &str =
    "/home/alex/avin/usr/connect/tinkoff/token.txt";

// Datetime
pub const DAY_BEGIN: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
pub const DAY_END: NaiveTime = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
pub const MSK_TIME_DIF: TimeDelta = TimeDelta::hours(3);
pub const MINUTES_IN_DAY: i32 = 24 * 60 * 60;

// TODO: del, use from config.toml
pub const DT_FMT: &str = "%Y-%m-%d %H:%M:%S";
