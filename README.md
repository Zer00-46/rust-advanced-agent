# Rust Advanced Agent

An advanced remote administration tool written in Rust for Windows, focusing on stealth and privilege escalation.

## Features
- **Obfuscated Strings**: Uses XOR encryption for critical strings (IPs, commands) to evade static analysis.
- **UAC Bypass**: Implements a registry-based bypass using `fodhelper.exe` to gain elevated privileges.
- **Process Hollowing**: Ability to inject shellcode into legitimate processes (e.g., `svchost.exe`) for camouflage.
- **Reverse Shell**: Persistent TCP connection for remote command execution.

## Setup

### Prerequisites
- Rust Toolchain (Cargo)
- Windows SDK

### Installation
1. Clone the repository: git clone https://github.com/Zer00-46/rust-advanced-agent.git

2. Navigate to the directory: cd rust-advanced-agent

3. Build the project: cargo build --release

Usage

    Run the compiled binary on the target machine.
    Use a listener (like Netcat) on the specified IP and port to receive the connection.
    Available special commands:
        privesc: Triggers the UAC Bypass mechanism.
        camouflage: Performs process hollowing into svchost.exe.
        exit: Terminates the session.

### Disclaimer

### This project is for educational purposes and authorized security testing only.
