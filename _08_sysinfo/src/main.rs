use std::collections::HashMap;

use colored::Colorize;
use sysinfo::{System, IS_SUPPORTED_SYSTEM};

fn check_supported_system() {
    if IS_SUPPORTED_SYSTEM {
        return;
    };

    eprintln!(
        "{} system not supported! Come back with a decent OS.",
        "ERROR:".blue()
    );

    std::process::exit(1)
}

fn main() {
    check_supported_system();

    let mut sys = System::new_all();

    let mut sys_info: HashMap<String, String> = HashMap::new();

    println!(
        "System      : {}",
        System::name().unwrap_or(String::from("unknown"))
    );
    println!(
        "OS Ver.     : {}",
        System::os_version().unwrap_or(String::from("unknown"))
    );
    println!(
        "Kernal Ver. : {}",
        System::kernel_version().unwrap_or(String::from("unknown"))
    );
    println!(
        "Hostname    : {}",
        System::host_name().unwrap_or(String::from("unknown"))
    );
    println!();
    println!(
        "Memory      : {} / {}",
        sys.used_memory(),
        sys.total_memory()
    );
    println!("Swap        : {} / {}", sys.used_swap(), sys.total_swap());
    println!();
    println!("CPUs        : {}", sys.cpus().len());
}
