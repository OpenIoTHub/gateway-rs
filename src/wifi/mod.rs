#[cfg(target_os = "espidf")]
mod ping {
    use anyhow::{bail, Error, Result};
    use embedded_svc::ipv4;
    use esp_idf_svc::ping;
    use log::*;

    pub fn ping(ip: ipv4::Ipv4Addr) -> Result<()> {
        info!("About to do some pings for {:?}", ip);

        let ping_summary = ping::EspPing::default().ping(ip, &Default::default())?;
        if ping_summary.transmitted != ping_summary.received {
            bail!("Pinging IP {} resulted in timeouts", ip);
        }

        info!("Pinging done");

        Ok(())
    }
}

#[cfg(target_os = "espidf")]
pub mod wifi {
    use esp_idf_sys::{self, EspError}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
    use std::time::Duration;

    use anyhow::{bail, Error, Result};
    // use embedded_svc::ipv4;
    use embedded_svc::wifi::*;
    use esp_idf_hal::peripheral;
    use esp_idf_hal::prelude::*;
    use esp_idf_svc::eventloop::*;
    use esp_idf_svc::netif::*;
    use esp_idf_svc::wifi::*;
    use log::*;

    use super::ping::ping;

    const SSID: &str = "Xiaomi_3g_1";
    // const SSID: &str = "wifi";
    const PASS: &str = "66668888abc";

    #[allow(dead_code)]
    pub(crate) fn wifi(
        modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
        sysloop: EspSystemEventLoop,
    ) -> Result<Box<EspWifi<'static>>> {
        use std::net::Ipv4Addr;

        use esp_idf_svc::handle::RawHandle;

        let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), None)?);

        info!("Wifi created, about to scan");

        let ap_infos = wifi.scan()?;

        let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

        let channel = if let Some(ours) = ours {
            info!(
                "Found configured access point {} on channel {}",
                SSID, ours.channel
            );
            Some(ours.channel)
        } else {
            info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
            None
        };

        wifi.set_configuration(&Configuration::Mixed(
            ClientConfiguration {
                ssid: SSID.into(),
                password: PASS.into(),
                channel,
                ..Default::default()
            },
            AccessPointConfiguration {
                ssid: "aptest".into(),
                channel: channel.unwrap_or(1),
                ..Default::default()
            },
        ))?;

        wifi.start()?;

        info!("Starting wifi...");

        if !WifiWait::new(&sysloop)?
            .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
        {
            bail!("Wifi did not start");
        }

        info!("Connecting wifi...");

        wifi.connect()?;

        if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
            Duration::from_secs(20),
            || {
                wifi.is_connected().unwrap()
                    && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
            },
        ) {
            bail!("Wifi did not connect or did not receive a DHCP lease");
        }

        let ip_info = wifi.sta_netif().get_ip_info()?;

        info!("Wifi DHCP info: {:?}", ip_info);

        ping(ip_info.subnet.gateway)?;

        Ok(wifi)
    }
}
