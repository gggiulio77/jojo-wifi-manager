use load_dotenv::load_dotenv;
use log::*;
use jojo_wifi_manager::{connect, scan, Network};

load_dotenv!();

const NETWORK_SSID_FILTER: &'static str = env!("NETWORK_SSID_FILTER");
const NETWORK_PASSWORD: &'static str = env!("NETWORK_PASSWORD");

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    info!("Start wifi manager");

    info!(
        "Filter: {:?}, Password: {:?}",
        NETWORK_SSID_FILTER, NETWORK_PASSWORD
    );

    let networks = scan()?;

    info!("networks list: {:?}", networks);

    // TODO: abstract struct
    let filtered_network: Vec<Network> = networks
        .into_iter()
        .filter(|net| net.ssid.contains(NETWORK_SSID_FILTER))
        .collect();

    let mouse_network = filtered_network.first().unwrap();

    connect(mouse_network.ssid.to_string(), NETWORK_PASSWORD.to_string())?;

    info!("Connected");

    Ok(())
}
