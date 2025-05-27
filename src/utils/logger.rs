use chrono::Local;
use log::{Metadata, Record};

pub static LOGGER: ConsoleLogger = ConsoleLogger;
pub struct ConsoleLogger;
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
