/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::utils;

fn main() -> eframe::Result {
    log::set_logger(&utils::LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");

    eframe::run_native(
        "AVIN - Terminal",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(avin::gui::Terminal::new(cc)))),
    )
}
