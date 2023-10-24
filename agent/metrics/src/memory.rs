use sysinfo::{System, SystemExt};

pub fn get_memory_usage(sys: &mut System) -> f32 {
    return (sys.used_memory() as f64 / sys.total_memory() as f64) as f32;
}

pub fn get_memory_swap_usage(sys: &mut System) -> f32 {
    return (sys.used_swap() as f64 / sys.total_swap() as f64) as f32;
}
