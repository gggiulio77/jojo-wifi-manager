# jojo-wifi-manager

This library provides basic functionalities for scanning and connecting to WiFi networks. It achieves this by interfacing with the operating system's WiFi module. It's utilized within the [jojo-otp-app](https://github.com/gggiulio77/jojo-otp-app) to search for and connect to the [jojo-client](https://github.com/gggiulio77/jojo-client) network.

### Quick Links

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [License](#license)

## Getting Started

To execute the project as a binary, utilize the `cargo run` command. The main.rs file serves as the entry point. Within this file, parameters are hardcoded to invoke the `scan`  and `connect` functions located inside lib.rs. You can set your network name and password with NETWORK_SSID_FILTER and NETWORK_PASSWORD .env variables.

### Prerequisites

Before proceeding, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Installation

`git clone https://github.com/gggiulio77/jojo-wifi-manager.git`

## Usage

Just add jojo-wifi-manager as a dependency in your `Cargo.toml`. Something like this: 
```
[dependencies]
...
jojo-wifi-manager = { git = "https://github.com/gggiulio77/jojo-wifi-manager.git" }
...
```

## Roadmap

- [ ] Improve error handling
- [ ] Move business models to [jojo-common](https://github.com/gggiulio77/jojo-common.git)

## License
