use regex::Regex;
use sysinfo::{NetworkExt, System, SystemExt};

#[derive(Debug)]
pub struct NetworkMetrics {
    pub received: u64,
    pub transmitted: u64,
    pub packet_received: u64,
    pub packet_transmitted: u64,
    pub error_on_received: u64,
    pub error_on_transmitted: u64,
    pub mac_address: String,
    pub interface_name: String,
}

pub fn get_network_interface_metric(sys: &mut System) -> Vec<NetworkMetrics> {
    // https://www.freedesktop.org/software/systemd/man/latest/systemd.net-naming-scheme.html
    let interface_name_to_include = vec!["en.*", "eth.*", "sl.*", "wl.*", "ww.*"];
    let match_regex_str = format!("^({})$", interface_name_to_include.join("|"));
    let match_regex = Regex::new(&match_regex_str).unwrap();

    let networks = sys.networks();

    let mut results: Vec<NetworkMetrics> = vec![];

    for (interface_name, data) in networks {
        if !match_regex.is_match(interface_name) {
            continue;
        }

        let network_metric: NetworkMetrics = NetworkMetrics {
            interface_name: interface_name.clone(),
            mac_address: data.mac_address().to_string(),
            received: data.received(),
            transmitted: data.transmitted(),
            packet_received: data.packets_received(),
            packet_transmitted: data.packets_transmitted(),
            error_on_received: data.errors_on_received(),
            error_on_transmitted: data.errors_on_transmitted(),
        };

        results.push(network_metric);
    }

    return results;
}
