# Hydro Controller

A bare metal Rust-based controller for hydroponic systems targeting the ESP32-C6 microcontroller.

## Features

- WiFi connectivity
- Bluetooth Low Energy support
- Network stack with TCP/IP
- Hardware GPIO control for sensor integration
- Async task management using Embassy runtime
- Optimized for embedded systems (no_std)

## Prerequisites

- Rust toolchain
- ESP32-C6 development board
- `espflash` tool for flashing

## Building

```bash
cargo build
```

## Flashing

```bash
espflash flash --monitor --chip esp32c6
```

## Development

This project is built using:
- Embassy async runtime for task management
- RISC-V architecture (riscv32imac-unknown-none-elf)
- Custom heap allocation (72KB)
- ESP-specific features (WiFi, BLE)

## License

This project is licensed under the GNU General Public License v3.0 (GPLv3) - see the [LICENSE](LICENSE) file for details.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
