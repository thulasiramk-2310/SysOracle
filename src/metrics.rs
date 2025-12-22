use sysinfo::System;

pub struct Metrics {
    pub cpu: f32,
    pub memory_used: u64,
    pub memory_total: u64,
}

impl Metrics {
    pub fn collect(system: &mut System) -> Self {
        system.refresh_cpu();
        system.refresh_memory();

        let cpu = system.global_cpu_info().cpu_usage();
        let memory_used = system.used_memory();
        let memory_total = system.total_memory();

        Self {
            cpu,
            memory_used,
            memory_total,
        }
    }
}
