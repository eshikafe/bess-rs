[![Build Status](https://github.com/eshikafe/bess-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/eshikafe/bess-rs/actions/workflows/rust.yml)


## bess-rs
A Rust implementation of [BESS(Berkeley Extensible Software Switch)](https://github.com/NetSys/bess).

## Dependencies
- Python 3+ (for bessctl)
- `sudo apt update && sudo apt upgrade -y`
- `sudo apt install -y protobuf-compiler libprotobuf-dev`
- `pip install --user protobuf==3.20.1 grpcio==1.46.0 grpcio-tools==1.46.0`
- `pip install --user scapy`

## Objective
Experimenting with Rust for fast packet processing.

## Usage
```=shell
RUST_LOG=debug cargo run --bin bessd -- --help
```

## Contributions
The main task right now is to port the C++ code base completely to Rust.
Please take any C++ file of your choice under the `core` folder and port it to Rust.

