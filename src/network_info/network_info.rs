use crate::network_info::network_stats::NetworkStats;
use std::io;

pub struct NetworkInfo {
    pub download_speed: f64,
    pub upload_speed: f64,
}

impl NetworkInfo {
    pub fn new() -> io::Result<Self> {
        let network_stats_before = NetworkStats::get_network_stats()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        let network_stats_after = NetworkStats::get_network_stats()?;

        let (rx_bytes_before, tx_bytes_before) = network_stats_before;
        let (rx_bytes_after, tx_bytes_after) = network_stats_after;

        let download_speed = ((rx_bytes_after - rx_bytes_before) as f64) / 1.0;
        let upload_speed = ((tx_bytes_after - tx_bytes_before) as f64) / 1.0;

        Ok(NetworkInfo {
            download_speed,
            upload_speed,
        })
    }
}
