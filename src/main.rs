use pcap::{Device, Capture};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Parse command-line arguments for device name and filter
    let args: Vec<String> = env::args().collect();

    // Get device name or use default
    let device_name = if args.len() > 1 {
        &args[1]
    } else {
        "default"
    };

    // Get filter expression or use default ("")
    let filter_expr = if args.len() > 2 {
        &args[2]
    } else {
        ""
    };

    // Open or create a log file
    let mut log_file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open("packet_log.txt")
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
            return;
        }
    };

    // Find the specified network device
    let device = match Device::list() {
        Ok(devices) => {
            if device_name == "default" {
                devices.into_iter().next()
            } else {
                devices.into_iter().find(|d| d.name == *device_name)
            }
        }
        Err(e) => {
            eprintln!("Failed to list devices: {}", e);
            return;
        }
    };

    let device = match device {
        Some(d) => d,
        None => {
            eprintln!("No matching device found or no devices available.");
            return;
        }
    };

    println!("Using device: {}", device.name);

    // Open a capture handle on the selected device
    let mut cap = match Capture::from_device(device)
        .unwrap()
        .open() // Promiscuous mode removed
    {
        Ok(capture) => capture,
        Err(e) => {
            eprintln!("Failed to open device for capture: {}", e);
            return;
        }
    };

    // Apply filter if specified
    if !filter_expr.is_empty() {
        if let Err(e) = cap.filter(filter_expr, true) {
            eprintln!("Failed to set filter '{}': {}", filter_expr, e);
            return;
        }
        println!("Filter applied: {}", filter_expr);
    } else {
        println!("No filter applied.");
    }

    // Loop to capture packets
    while let Ok(packet) = cap.next_packet() {
        let log_entry = format!(
            "Captured packet:\n    Data: {:?}\n    Length: {} bytes\n",
            packet.data,
            packet.header.len
        );

        // Print to console
        println!("{}", log_entry);

        // Write to log file
        if let Err(e) = writeln!(log_file, "{}", log_entry) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }
}

