## All Rust libraries and binaries created by Ben Santora. All were coded using AI: ChatGPT - Widenex Rust GPT - Microsoft CoPilot

## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# oxide_linux2

`oxide_linux2` is a Rust library that provides utilities for retrieving system information on Linux. It allows users to easily fetch details like kernel version, system uptime, available memory, CPU information, load averages, and disk usage.

## Features

- Retrieve Linux kernel version
- Fetch system uptime
- Get available memory in kilobytes
- Retrieve CPU information
- Fetch system load averages
- Get disk usage statistics

## Installation

To use `oxide_linux2` in your project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
oxide_linux2 = "0.1.5"
thiserror = "1.0"
nix = "0.23.0"
```
## usage
Here's an example of how to use the library:

```mod oxide_linux2;

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
```
## License
This project is licensed under the MIT License

## Author
Ben Santora 
