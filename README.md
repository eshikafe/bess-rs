[![Build Status](https://github.com/eshikafe/bess-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/eshikafe/bess-rs/actions/workflows/rust.yml)


## bess-rs
A Rust implementation of [BESS(Berkeley Extensible Software Switch)](https://github.com/NetSys/bess).

## Dependencies
- `sudo apt update && sudo apt upgrade -y`
- `sudo apt install -y protobuf-compiler libprotobuf-dev`
- Install python3 (Python 3.8.10 used for development)
- `pip install --user protobuf==3.20.1 grpcio==1.46.0 grpcio-tools==1.46.0`
- `pip install --user scapy`

## Objective
Experimenting with Rust for high performance packet processing.

## Contributions
The main task right now is to port the C++ code base completely to Rust.
Please take any C++ file of your choice under the `core` folder and port it to Rust.

## Resources
[A Guide to Porting C/C++ to Rust](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/)
