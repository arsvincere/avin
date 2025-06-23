/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::Local;
use log::{Metadata, Record};

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");
}

static LOGGER: ConsoleLogger = ConsoleLogger;
struct ConsoleLogger;
impl log::Log for ConsoleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // metadata.level() <= log::Level::Info
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} [{}] {}",
                Local::now().format("%H:%M:%S"),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
