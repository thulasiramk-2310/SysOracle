use sysinfo::{Disks, Networks, System};

#[derive(Clone)]
pub struct CoreUsage {
    pub name: String,
    pub usage: f32,
}

#[derive(Clone)]
pub struct GpuInfo {
    pub name: String,
    pub usage: f32,
    pub memory_used: u64,  // bytes
    pub memory_total: u64, // bytes
}

#[derive(Clone)]
pub struct DiskInfo {
    #[allow(dead_code)]
    pub name: String,
    pub mount_point: String,
    pub used: u64,  // bytes
    pub total: u64, // bytes
}

#[derive(Clone)]
pub struct Metrics {
    pub cpu: f32,
    pub cpu_cores: Vec<CoreUsage>,
    pub memory_used: u64,
    pub memory_total: u64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub network_rx_speed: f32,  // MB/s
    pub network_tx_speed: f32,  // MB/s
    pub gpu: Option<GpuInfo>,
    pub disks: Vec<DiskInfo>,
    pub uptime: u64, // seconds
}

impl Metrics {
    pub fn collect(system: &mut System, prev_rx: u64, prev_tx: u64, refresh_rate_ms: u64) -> Self {
        // Note: system.refresh_*() methods are called selectively in app.rs before calling this
        // This avoids double refresh and ensures proper CPU timing
        
        let cpu = system.global_cpu_info().cpu_usage();
        let memory_used = system.used_memory();
        let memory_total = system.total_memory();
        let uptime = System::uptime();

        // Per-core CPU usage
        let cpu_cores: Vec<CoreUsage> = system
            .cpus()
            .iter()
            .map(|cpu| CoreUsage {
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
            })
            .collect();

        // Network stats
        let networks = Networks::new_with_refreshed_list();
        let mut total_rx = 0;
        let mut total_tx = 0;

        for (_interface_name, data) in &networks {
            total_rx += data.total_received();
            total_tx += data.total_transmitted();
        }

        // Calculate network speed (bytes/ms -> MB/s)
        let rx_delta = total_rx.saturating_sub(prev_rx);
        let tx_delta = total_tx.saturating_sub(prev_tx);
        
        let time_sec = refresh_rate_ms as f32 / 1000.0;
        let network_rx_speed = if time_sec > 0.0 {
            (rx_delta as f32 / time_sec) / 1024.0 / 1024.0
        } else {
            0.0
        };
        let network_tx_speed = if time_sec > 0.0 {
            (tx_delta as f32 / time_sec) / 1024.0 / 1024.0
        } else {
            0.0
        };

        // GPU monitoring (optional)
        let gpu = Self::collect_gpu_info();

        // Disk usage
        let disks = Self::collect_disk_info();

        Self {
            cpu,
            cpu_cores,
            memory_used,
            memory_total,
            network_rx: total_rx,
            network_tx: total_tx,
            network_rx_speed,
            network_tx_speed,
            gpu,
            disks,
            uptime,
        }
    }

    #[cfg(feature = "gpu")]
    fn collect_gpu_info() -> Option<GpuInfo> {
        use nvml_wrapper::Nvml;
        
        match Nvml::init() {
            Ok(nvml) => {
                match nvml.device_count() {
                    Ok(count) if count > 0 => {
                        match nvml.device_by_index(0) {
                            Ok(device) => {
                                let name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());
                                let usage = device.utilization_rates()
                                    .map(|u| u.gpu as f32)
                                    .unwrap_or(0.0);
                                    
                                if let Ok(memory_info) = device.memory_info() {
                                    Some(GpuInfo {
                                        name,
                                        usage,
                                        memory_used: memory_info.used,
                                        memory_total: memory_info.total,
                                    })
                                } else {
                                    eprintln!("[SysOracle] GPU memory info unavailable");
                                    None
                                }
                            }
                            Err(e) => {
                                eprintln!("[SysOracle] Failed to get GPU device: {:?}", e);
                                None
                            }
                        }
                    }
                    Ok(_) => {
                        eprintln!("[SysOracle] No NVIDIA GPUs detected");
                        None
                    }
                    Err(e) => {
                        eprintln!("[SysOracle] Failed to get GPU count: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("[SysOracle] NVML initialization failed: {:?}", e);
                None
            }
        }
    }

    #[cfg(not(feature = "gpu"))]
    fn collect_gpu_info() -> Option<GpuInfo> {
        None
    }

    fn collect_disk_info() -> Vec<DiskInfo> {
        let disks = Disks::new_with_refreshed_list();
        
        disks
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                used: disk.total_space() - disk.available_space(),
                total: disk.total_space(),
            })
            .collect()
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
