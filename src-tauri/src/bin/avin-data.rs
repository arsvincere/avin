/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_data::Cli;

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    Cli::run().await;
}
