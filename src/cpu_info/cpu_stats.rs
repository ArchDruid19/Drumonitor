use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct CpuStats {
    stats: Vec<i64>,
}

impl CpuStats {
    pub fn new() -> io::Result<Self> {
        let stats = Self::read_cpu_stats()?;
        Ok(CpuStats { stats })
    }

    fn read_cpu_stats() -> io::Result<Vec<i64>> {
        let mut reader = BufReader::new(File::open("/proc/stat")?);

        let mut line = String::new();
        reader.read_line(&mut line)?;

        // Assuming we only need the first 10 items of the first line
        let cpu_stats: Vec<i64> = line
            .split_whitespace()
            .skip(1) // Skip over cpu name
            .take(10)
            .filter_map(|part| part.parse().ok())
            .collect();

        Ok(cpu_stats)
    }

    pub fn get_stats(&self) -> &[i64] {
        &self.stats
    }
}
