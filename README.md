[![Build Status](https://github.com/eshikafe/bess-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/eshikafe/bess-rs/actions/workflows/rust.yml)


## bess-rs
An implementation of [BESS](https://github.com/NetSys/bess) in Rust.

## Dependencies
- Python 3+ (for bessctl)
- `sudo apt update && sudo apt upgrade -y`
- `sudo apt install -y protobuf-compiler libprotobuf-dev`
- `pip install --user protobuf==3.20.1 grpcio==1.46.0 grpcio-tools==1.46.0`
- `pip install --user scapy`

## Objective
A high-performance and memory-safe data plane for 5G.

## Usage
```=shell
RUST_LOG=debug cargo run --bin bessd -- --help
```

## Contributions
The main task right now is to port the C++ code base completely to Rust.
Please take any C++ file of your choice under the `core` folder and port it to Rust.

