// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("src/tbank/api")?;

    tonic_prost_build::configure()
        .build_server(false)
        .out_dir("src/tbank/api")
        .compile_protos(
            &[
                "src/tbank/proto/common.proto",
                "src/tbank/proto/instruments.proto",
                "src/tbank/proto/marketdata.proto",
                "src/tbank/proto/operations.proto",
                "src/tbank/proto/orders.proto",
                "src/tbank/proto/sandbox.proto",
                "src/tbank/proto/signals.proto",
                "src/tbank/proto/stoporders.proto",
                "src/tbank/proto/users.proto",
            ],
            &["src/tbank/proto"],
        )?;

    Ok(())
}
