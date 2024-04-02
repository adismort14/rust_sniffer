# Rust IP Sniffer

This weekend project of mine is one of the projects of my `get-comfy-with-rust series`.   
It is a simple tool designed to scan for open ports on a given IP address. It utilizes multi-threaded parallelism for efficient scanning.

### Usage
The program supports three types of arguments:
- `ip_sniffer -h`: Displays usage instructions.
- `ip_sniffer <IP_ADDRESS>`: Scans for open ports on the specified IP address using default settings.
- `ip_sniffer -t <NUM_THREADS> <IP_ADDRESS>`: Scans for open ports on the specified IP address using a custom number of threads.
  
Example usage:

- `ip_sniffer -h`: Displays usage instructions.
- `ip_sniffer 192.168.1.1`: Scans for open ports on the IP address 192.168.1.1 using default settings.
- `ip_sniffer -t 16 192.168.1.1`: Scans for open ports on the IP address 192.168.1.1 using 16 threads.

### To run
- Clone the repo: `git clone https://github.com/adismort14/rust_sniffer`
- `cd rust_sniffer`
- `cargo build --release`
