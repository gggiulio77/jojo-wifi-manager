use anyhow::bail;
use wifi_rs::prelude::*;

// TODO: move this to jojo_common
#[derive(Debug, Clone)]
pub struct Network {
    /// mac address
    pub mac: String,
    /// hotspot name
    pub ssid: String,
    pub channel: String,
    /// wifi signal strength in dBm
    pub signal_level: String,
    /// this field is currently empty in the Linux version of the lib
    pub security: String,
}

impl Into<Network> for wifiscanner::Wifi {
    fn into(self) -> Network {
        let wifiscanner::Wifi {
            mac,
            ssid,
            channel,
            signal_level,
            security,
        } = self;

        Network {
            mac,
            ssid,
            channel,
            signal_level,
            security,
        }
    }
}

// TODO: create better errors
pub fn scan() -> anyhow::Result<Vec<Network>> {
    match wifiscanner::scan() {
        Ok(wifi_vec) => {
            let networks: Vec<Network> = wifi_vec.into_iter().map(|wifi| wifi.into()).collect();
            Ok(networks)
        }
        Err(err) => bail!("Error while scanning: {:?}", err),
    }
}

// TODO: create better errors
// TODO: replace with SSid and Password from jojo_common::network
pub fn connect(ssid: String, password: String) -> anyhow::Result<()> {
    let config = Some(wifi_rs::prelude::Config {
        interface: Some("wlo1"),
    });

    let mut wifi = wifi_rs::WiFi::new(config);

    match wifi.connect(&ssid, &password) {
        // TODO: this bool is bugged as fuck
        Ok(result) => {
            if result {
                Ok(())
            } else {
                bail!("Cannot connect")
            }
        }
        Err(err) => bail!("Error while connecting: {:?}", err),
    }
}
