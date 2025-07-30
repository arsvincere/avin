use avin::utils;

fn main() -> eframe::Result {
    utils::init_logger();

    eframe::run_native(
        "AVIN - Tester",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(avin::gui::Tester::new(cc)))),
    )
}
