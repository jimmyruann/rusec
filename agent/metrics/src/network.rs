use regex::Regex;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};

pub fn get_network_interface_metric(sys: &mut System) {
    // https://www.freedesktop.org/software/systemd/man/latest/systemd.net-naming-scheme.html
    let interface_name_to_include = vec!["en.*", "eth.*", "sl.*", "wl.*", "ww.*"];
    let match_regex_str = format!("^({})$", interface_name_to_include.join("|"));
    let match_regex = Regex::new(&match_regex_str).unwrap();

    let networks = sys.networks();

    for (interface_name, data) in networks {
        if !match_regex.is_match(interface_name) {
            continue;
        }
        println!("{}", interface_name);

        println!("[{:?}] {:?}", interface_name, data);
    }
}
