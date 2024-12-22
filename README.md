# Network Sniffer

This project is a simple network sniffer built in Rust, leveraging the `pcap` library to capture network packets and log them to a file.

## Project Overview

The Network Sniffer captures packets from a specified network device or the default device, filters them based on user input (if provided), and logs the packet data and length to a text file (`packet_log.txt`).

## Development Environment and Dependencies

- **Development Environment**:
  - Rust (Edition 2021)
  - Compatible with Linux, macOS, and Windows systems (ensure the system has the required permissions and dependencies for `pcap`).

- **Dependencies**:
  - `pcap` crate, version `0.10` (for handling packet capturing).

Ensure you have `libpcap` installed on your system. On Linux, you can install it using your package manager (e.g., `sudo apt install libpcap-dev` on Debian-based systems).

## How to Compile

1. Install Rust and Cargo by following the instructions on [rustup.rs](https://rustup.rs/).
2. Clone the project repository or copy the code into a directory.
3. Navigate to the project directory:
   ```sh
   cd network_sniffer
   ```
4. Compile the project:
   ```sh
   cargo build
   ```
   The compiled binary will be located in the `target/debug` directory.


## How to Run

1. Run the program with optional arguments for the network device and filter expression:
   ```sh
   cd target/debug
   sudo ./network_sniffer <device_name> <filter_expression>
   ```
   - `<device_name>`: Name of the network device to capture packets from. If not provided, the default device will be used.
   - `<filter_expression>`: A filter expression to capture specific packets (e.g., `tcp port 80`). If not provided, all packets will be captured.

2. Example usage:
   ```sh
   sudo ./network_sniffer eth0 "tcp port 80"
   ```
   This will capture TCP packets on port 80 from the `eth0` device.

3. The captured packets will be logged to `packet_log.txt` in the current working directory.

## Notes

- Ensure you have sufficient permissions to access network devices. On Linux, you may need to run the program as `root` or use `sudo`.
- If `pcap` is not installed, the program will fail to build or run. Install `libpcap` using your package manager.
- The program uses the `pcap` crate to list and capture packets. For additional features or troubleshooting, refer to the [pcap crate documentation](https://docs.rs/pcap/).

## Troubleshooting

- **Error: "Failed to list devices"**: Ensure that `libpcap` is installed and the program is run with appropriate permissions.
- **Error: "Failed to open device for capture"**: Verify that the specified device exists and is accessible.
- **No packets captured**: Ensure the device is active and connected to a network. Double-check the filter expression for accuracy.

