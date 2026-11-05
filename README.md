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
1. Clone the repository: git clone https://github.com/youruser/rust-advanced-agent.git

Navigate to the directory: cd rust-advanced-agent

3. Build the project: cargo build --release
