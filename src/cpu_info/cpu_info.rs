use crate::cpu_info::cpu_stats::CpuStats;
use std::io;
pub struct CpuInfo {
    stats_before: CpuStats,
    stats_after: CpuStats,
}

impl CpuInfo {
    pub fn new() -> io::Result<Self> {
        let stats_before = CpuStats::new()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        let stats_after = CpuStats::new()?;

        Ok(CpuInfo {
            stats_before,
            stats_after,
        })
    }

    pub fn cpu_usage(&self) -> f64 {
        let cpu_total_before = self.stats_before.get_stats();
        let cpu_total_after = self.stats_after.get_stats();

        let cpu_total_delta: i64 = cpu_total_after
            .iter()
            .zip(cpu_total_before)
            .map(|(a, b)| a - b)
            .sum();

        let cpu_idle_delta = cpu_total_after[3] - cpu_total_before[3];

        if cpu_total_delta > 0 {
            let cpu_non_idle_delta = cpu_total_delta - cpu_idle_delta;
            (cpu_non_idle_delta as f64 / cpu_total_delta as f64) * 100.0
        } else {
            eprintln!("CPU delta is less than 1!");
            0.0
        }
    }
}
