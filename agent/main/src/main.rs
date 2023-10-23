use sysinfo::{NetworkExt, System, SystemExt};

use metrics::cpu;
use metrics::memory;
use metrics::network;
use metrics::system::{get_composed_system_info, SystemInfo};

fn main() {
    println!("Hello, world!");
    let mut sys: System = System::new_all();

    println!("Current System supported: {}", System::IS_SUPPORTED);

    let infos: SystemInfo = get_composed_system_info(&mut sys);

    // println!("{:?}", infos);

    // println!("{:?}", cpu::get_cpu_usage(&mut sys));

    // println!("{:?}", memory::get_memory_usage(&mut sys));
    // println!("{:?}", memory::get_memory_swap_usage(&mut sys));

    network::get_network_interface_metric(&mut sys);
}
