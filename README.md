# Data-plane (bess-rs)
A dataplane implementation in Rust based on the [BESS](https://github.com/NetSys/bess) architecture for high performance packet processing.
Main use case: an experimental 5G UPF data-plane.

## Objective
An experiment with Rust for high performance packet processing.

## Contributions
The main task right now is to port the C++ code base completely to Rust.
Please take any C++ file of your choice under the `core` folder and port it to Rust.

## Notes
These are some of the porting strategies (from C++ to Rust) that I am using for this project. It is not an exhaustive list.
- Use `clap` in place of `#include <gflags/gflags.h>`
- Use `log` and `env_logger` in place of `#include <glog/logging.h>`
- C++ `namespace bess`  => Rust `mod bess_rs`. Same convention should be used for other namespaces.
- `std::cout << .. << endl;` => `println!(..);`

## Resources
[A Guide to Porting C/C++ to Rust](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/)
