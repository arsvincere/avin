use avin_utils as utils;

fn main() -> eframe::Result {
    utils::init_logger();

    eframe::run_native(
        "AVIN - Terminal",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(avin::gui::GuiSimulator::new(cc)))),
    )
}
