use anyhow::{bail, Ok, Result};
use esp_idf_svc::nvs::{self, EspDefaultNvsPartition};
use esp_idf_sys::{self, EspError};
use log::*; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

pub fn init() -> Result<()> {
    esp_idf_sys::link_patches();
    // log
    esp_idf_svc::log::EspLogger::initialize_default();
    #[allow(clippy::needless_update)]
    {
        esp_idf_sys::esp!(unsafe {
            esp_idf_sys::esp_vfs_eventfd_register(&esp_idf_sys::esp_vfs_eventfd_config_t {
                max_fds: 50,
                ..Default::default()
            })
        })?;
    }
    // // 初始化nvs存储
    EspDefaultNvsPartition::take()?;
    Ok(())
}
