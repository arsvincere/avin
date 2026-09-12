// ───────────────────────────── 2023.07.23 15:06 ─────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod cli;

use std::process::ExitCode;

use cli::AvinCli;

#[tokio::main]
async fn main() -> ExitCode {
    match AvinCli::run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err.report());
            ExitCode::FAILURE
        }
    }
}
