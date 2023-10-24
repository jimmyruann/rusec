use sysinfo::{System, SystemExt};

use metrics::cpu;
use metrics::disk;
use metrics::memory;
use metrics::network;
use metrics::system;

fn main() {
    println!("Hello, world!");
    let mut sys: System = System::new_all();

    println!("Current System supported: {}", System::IS_SUPPORTED);

    disk::get_disks(&mut sys);

    // println!("{:?}", system::get_composed_system_info(&mut sys));

    // println!("{:?}", cpu::get_cpu_usage(&mut sys));

    // println!("{:?}", memory::get_memory_usage(&mut sys));
    // println!("{:?}", memory::get_memory_swap_usage(&mut sys));
    // println!("{:?}", network::get_network_interface_metric(&mut sys));
}
