use sysinfo::{Cpu, CpuExt, System, SystemExt};

#[derive(Debug)]
pub struct CPUInfo {
    pub name: String,
    pub physical_core_count: usize,
    pub logical_core_count: usize,
    pub frequency: u64,
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub total: u64,
    pub swap: u64,
}

#[derive(Debug)]
pub struct SystemInfo {
    pub os_name: String,
    pub cpu: CPUInfo,
    pub memory: MemoryInfo,
}

pub fn get_composed_system_info(sys: &mut System) -> SystemInfo {
    let global_cpu_info: &Cpu = sys.global_cpu_info();
    let cpus: &[Cpu] = sys.cpus();

    return SystemInfo {
        os_name: vec![
            sys.name().unwrap_or("".to_string()),
            sys.os_version().unwrap_or("".to_string()),
            sys.kernel_version().unwrap_or("".to_string()),
        ]
        .join(" "),
        cpu: CPUInfo {
            name: global_cpu_info.name().to_string(),
            frequency: global_cpu_info.frequency(),
            physical_core_count: sys.physical_core_count().unwrap_or(0),
            logical_core_count: cpus.len(),
        },
        memory: MemoryInfo {
            total: sys.total_memory(),
            swap: sys.total_swap(),
        },
    };
}

pub fn get_uptime(sys: &mut System) -> u64 {
    return sys.uptime();
}
