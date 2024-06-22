use clap::{Arg, ArgAction, Command};
use human_bytes::human_bytes;
use std::thread::sleep;
use std::time::Duration;

mod cpu_info {
    pub mod cpu_info;
    pub mod cpu_stats;
}
mod ram_info {
    pub mod ram_info;
    pub mod ram_stats;
}
mod network_info {
    pub mod network_info;
    pub mod network_stats;
}

use cpu_info::cpu_info::CpuInfo;
use network_info::network_info::NetworkInfo;
use ram_info::ram_info::MemoryInfo;

fn main() {
    // Declare arguments using clap
    let matches = Command::new("drumon")
        .subcommand(
            // Cpu commands
            Command::new("cpu")
                .arg(Arg::new("percentage").short('p').action(ArgAction::SetTrue))
                .arg(
                    Arg::new("interval")
                        .short('i')
                        .value_parser(clap::value_parser!(u64))
                        .default_value("5"),
                ),
        )
        .subcommand(
            // Ram commands
            Command::new("ram")
                .arg(Arg::new("percentage").short('p').action(ArgAction::SetTrue))
                .arg(
                    Arg::new("usage")
                        .short('u')
                        .value_parser(clap::value_parser!(String))
                        .default_value("/") // Default to this seperator when none is supplied
                        .num_args(0..=1)
                        .default_missing_value("/"),
                )
                .arg(
                    Arg::new("interval")
                        .short('i')
                        .value_parser(clap::value_parser!(u64))
                        .default_value("5"),
                ),
        )
        .subcommand(
            // Network commands
            Command::new("network")
                .arg(Arg::new("traffic").short('t').action(ArgAction::SetTrue))
                .arg(
                    Arg::new("interval")
                        .short('i')
                        .value_parser(clap::value_parser!(u64))
                        .default_value("5"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("cpu", sub_m)) => {
            // Print cpu usage percentage
            if sub_m.get_flag("percentage") {
                run_monitor(sub_m, || {
                    CpuInfo::new().map(|info| format!("{:.1}%", info.cpu_usage()))
                });
            }
        }
        Some(("ram", sub_m)) => {
            // Print ram usage percentage
            if sub_m.get_flag("percentage") {
                run_monitor(sub_m, || {
                    MemoryInfo::new().map(|info| format!("{:.1}%", info.mem_percentage()))
                });
                // Print memory usage out of the total, auto converting with human_bytes
            } else {
                let seperator = sub_m.get_one::<String>("usage").unwrap();
                run_monitor(sub_m, || {
                    MemoryInfo::new().map(|info| {
                        format!(
                            "{} {} {}",
                            human_bytes(info.mem_usage() as f64),
                            seperator,
                            human_bytes(info.mem_total as f64)
                        )
                    })
                });
            }
        }
        Some(("network", sub_m)) => {
            // Print network upload and download speed
            if sub_m.get_flag("traffic") {
                run_monitor(sub_m, || {
                    NetworkInfo::new().map(|info| {
                        /* Transforms NetworkInfo struct into a formatted string
                         * and then transforms the 'Ok' values from the
                         * run_monitor into a Ok(String). */
                        format!(
                            "UP {} DN {}",
                            human_bytes(info.upload_speed as f64),
                            human_bytes(info.download_speed as f64)
                        )
                    })
                });
            }
        }
        _ => println!("Error! You must enter either cpu, ram, or network!"), // Change this messege
                                                                             // to call a help menu
    }
}
/* This takes the sub_m arg 'i' (if the user passes it) and get_info
 * the default interval value is 5. */
fn run_monitor<F>(sub_m: &clap::ArgMatches, get_info: F)
/* get_info is what is returned
 * from the generic paramter 'F'
 * and returns either Ok(String) or
 * Err(std::io::Error)*/
where
    F: Fn() -> Result<String, std::io::Error>,
{
    let interval = *sub_m.get_one::<u64>("interval").unwrap_or(&5);
    // Loop until the user closes
    loop {
        match get_info() {
            Ok(info) => println!("{}", info),
            Err(e) => eprintln!("Error! {}", e),
        }
        sleep(Duration::from_secs(interval));
    }
}
