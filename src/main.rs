use std::net::{TcpListener, TcpStream};
use std::time::Instant;
use std::{
    thread::{self, sleep},
    time::{self, Duration, SystemTime},
};

use anyhow::{bail, Error, Result};
use log::*;
use smol;
use smol::{io, Async};

mod client;
mod handle;
mod handle_msg;
mod login;
mod models;
mod test;
mod utils;
use test::*;
#[cfg(target_os = "espidf")]
mod init;
#[cfg(target_os = "espidf")]
mod wifi;

fn main() -> Result<()> {
    #[cfg(target_os = "espidf")]
    use esp_idf_hal::peripheral;
    #[cfg(target_os = "espidf")]
    use esp_idf_hal::prelude::*;
    #[cfg(target_os = "espidf")]
    use esp_idf_svc::eventloop::*;
    #[cfg(target_os = "espidf")]
    use wifi::wifi::wifi;
    #[cfg(target_os = "espidf")]
    init::init()?;
    #[cfg(target_os = "espidf")]
    // wifi
    let peripherals = Peripherals::take().unwrap();
    #[cfg(target_os = "espidf")]
    let sysloop = EspSystemEventLoop::take().unwrap();
    #[cfg(target_os = "espidf")]
    let mut wifi = wifi(peripherals.modem, sysloop.clone());

    test();

    client::gateway_manager::add_gateway();

    println!("main thread end!");
    loop {
        sleep(Duration::from_secs(3600));
    }
    Ok(())
}
