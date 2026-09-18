// ───────────────────────────── 2023.07.23 15:06 ─────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod cli;

use std::process::ExitCode;

use cli::AvinCli;

const ERROR: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";

#[tokio::main]
async fn main() -> ExitCode {
    match AvinCli::run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{ERROR}error:{RESET} {}", err.report());
            ExitCode::FAILURE
        }
    }
}
