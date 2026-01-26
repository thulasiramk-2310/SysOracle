use sysinfo::System;

#[derive(Clone)]
pub struct Metrics {
    pub cpu: f32,
    pub memory_used: u64,
    pub memory_total: u64,
}

impl Metrics {
    pub fn collect(system: &mut System) -> Self {
        system.refresh_all();  

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

#[derive(Clone)]
pub struct ProcInfo {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub mem: f32,
}

pub fn top_processes(system: &System, limit: usize) -> Vec<ProcInfo> {
    let mut list: Vec<ProcInfo> = system
        .processes()
        .values()
        .map(|p| ProcInfo {
            pid: p.pid().as_u32() as i32,
            name: p.name().to_string(),
            cpu: p.cpu_usage(),
            mem: if system.total_memory() > 0 {
                (p.memory() as f32 / system.total_memory() as f32) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    list.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());
    list.truncate(limit);

    list
}
