use sysinfo::{Cpu, CpuExt, System, SystemExt};

pub fn get_cpu_usage(sys: &mut System) -> f32 {
    let global_cpu: &Cpu = sys.global_cpu_info();

    global_cpu.cpu_usage()
}
