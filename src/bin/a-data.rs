/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::Command;
use avin::utils;
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    log::set_logger(&utils::LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");

    let args: Vec<String> = env::args().collect();

    let command: Command = match Command::build(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error arguments: {e}");
            process::exit(1);
        }
    };

    let result = command.execute().await;
    match result {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("Application error: {err}");
            process::exit(1)
        }
    }
}
