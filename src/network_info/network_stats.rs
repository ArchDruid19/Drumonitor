use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct NetworkStats {
    // Define fields as needed for storing network stats
}

impl NetworkStats {
    pub fn get_network_stats() -> io::Result<(i64, i64)> {
        let file = File::open("/proc/net/dev")?;
        let reader = BufReader::new(file);

        let mut rx_bytes = 0;
        let mut tx_bytes = 0;

        for line in reader.lines() {
            let line = line?;
            if line.contains("eth0") || line.contains("enp5s0") {
                // Change this to match your network interface
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 9 {
                    rx_bytes = parts[1].parse().unwrap_or(0);
                    tx_bytes = parts[9].parse().unwrap_or(0);
                }
            }
        }

        Ok((rx_bytes, tx_bytes))
    }
}
