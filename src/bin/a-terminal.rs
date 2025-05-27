/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

fn main() -> eframe::Result {
    eframe::run_native(
        "AVIN - Terminal",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(avin::gui::Terminal::new(cc)))),
    )
}
