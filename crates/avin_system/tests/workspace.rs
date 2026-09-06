// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_system::WORKSPACE;

#[test]
fn workspace_init() {
    WORKSPACE.init().unwrap();

    log::info!("Hello from AVIN");
}
