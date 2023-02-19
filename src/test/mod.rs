use anyhow::{Ok, Result};
use log::info;

pub fn test() -> Result<()> {
    if cfg!(target_os = "espidf") {
        info!("espidf");
    } else {
        info!("other target_os");
    }
    Ok(())
}
