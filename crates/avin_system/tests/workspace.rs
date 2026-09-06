// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_system::Workspace;

#[test]
#[ignore = "requires real AVIN workspace"]
fn workspace_init() {
    let _ = Workspace::get().unwrap();

    log::info!("Hello from AVIN");
}
