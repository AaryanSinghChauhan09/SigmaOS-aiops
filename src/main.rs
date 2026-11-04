//! SigmaOS-aiops - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-aiops");
    println!("SigmaOS SigmaOS-aiops v0.1.0");
    Ok(())
}
