use sysinfo::{DiskExt, System, SystemExt};

#[derive(Debug)]
pub struct DiskMetrics {
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
    pub space_usage: f32,
}

pub fn get_disks(sys: &mut System) -> Vec<DiskMetrics> {
    let disks = sys.disks();

    let results: Vec<DiskMetrics> = vec![];

    for disk in disks {
        // Ignore USB, removable drives
        if disk.is_removable() {
            continue;
        }

        println!("{:?}", disk.kind());

        // let disk_metric = DiskMetrics {
        //     file_system: disk.file_system()
        // };

        // results.push(disk_metric);
    }

    return results;
}
