use avin::utils;

fn main() -> eframe::Result {
    utils::init_logger();

    eframe::run_native(
        "AVIN - Scanner",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(avin::gui::Scanner::new(cc)))),
    )
}
