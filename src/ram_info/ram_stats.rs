use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct RamStats {
    pub stats: Vec<u32>,
}

impl RamStats {
    pub fn new() -> io::Result<Self> {
        let stats = Self::read_ram_stats()?;
        Ok(RamStats { stats })
    }

    fn read_ram_stats() -> io::Result<Vec<u32>> {
        let reader = BufReader::new(File::open("/proc/meminfo")?);
        let mut memory_information: Vec<u32> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
            if parts.len() >= 2 {
                memory_information.push(parts[0].parse::<u32>().unwrap_or(0));
            }
        }

        Ok(memory_information)
    }
}
