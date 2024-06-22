use crate::ram_info::ram_stats::RamStats;
use std::io;

pub struct MemoryInfo {
    pub mem_total: u32,
    pub mem_free: u32,
    pub mem_buffers: u32,
    pub mem_cached: u32,
}

impl MemoryInfo {
    pub fn mem_percentage(&self) -> f64 {
        (self.mem_usage() as f64 / self.mem_total as f64) * 100.0
    }

    pub fn mem_usage(&self) -> u32 {
        self.mem_total - self.mem_free - self.mem_buffers - self.mem_cached
    }

    pub fn new() -> io::Result<Self> {
        let memory_stats = RamStats::new()?;
        if memory_stats.stats.len() >= 5 {
            Ok(MemoryInfo {
                mem_total: memory_stats.stats[0],
                mem_free: memory_stats.stats[1],
                mem_buffers: memory_stats.stats[3],
                mem_cached: memory_stats.stats[4],
            })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data in /proc/meminfo",
            ))
        }
    }
}
