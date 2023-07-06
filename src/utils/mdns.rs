#[cfg(target_os = "espidf")]
mod mdns {
    // use esp_idf_svc::mdns::*;
}
#[cfg(not(target_os = "espidf"))]
mod mdns {}
