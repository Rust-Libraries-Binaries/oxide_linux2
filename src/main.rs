use oxide_linux2::SystemInfo;

fn main() {
    // Kernel version
    if let Some(kernel_version) = SystemInfo::kernel_version() {
        println!("Kernel Version: {}", kernel_version);
    } else {
        println!("Failed to get Kernel Version.");
    }

    // System uptime
    match SystemInfo::system_uptime() {
        Ok(uptime) => println!("System Uptime: {:.2} seconds", uptime),
        Err(e) => println!("Failed to get System Uptime: {}", e),
    }

    // Available memory
    match SystemInfo::available_memory() {
        Ok(memory) => println!("Available Memory: {} kB", memory),
        Err(e) => println!("Failed to get Available Memory: {}", e),
    }

    // CPU information
    match SystemInfo::cpu_info() {
        Ok(cpu_info) => println!("CPU Information: \n{}", cpu_info),
        Err(e) => println!("Failed to get CPU Information: {}", e),
    }

    // Load average
    match SystemInfo::load_average() {
        Ok(loadavg) => println!("Load Average: {:.2}, {:.2}, {:.2}", loadavg[0], loadavg[1], loadavg[2]),
        Err(e) => println!("Failed to get Load Average: {}", e),
    }

    // Disk usage
    match SystemInfo::disk_usage("/") {
        Ok((total, free)) => println!("Disk Usage: Total - {} bytes, Free - {} bytes", total, free),
        Err(e) => println!("Failed to get Disk Usage: {}", e),
    }
}
