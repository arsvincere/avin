fn main() -> eframe::Result {
    avin_utils::init_logger();

    eframe::run_native(
        "AVIN - Terminal",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(avin_gui::GuiSimulator::new(cc)))),
    )
}
