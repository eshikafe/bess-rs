use crate::bessd;
use clap::Parser;
use log::*;

// Port this BESS instance listens on.
// Panda came up with this default number
pub const K_DEFAULT_PORT: u32 = 10514;
pub const K_DEFAULT_BIND_ADDR: &str = "127.0.0.1";

/// BESS Command Line Options:
#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Options {
    /// Dump the size of internal data structures
    #[clap(long, value_parser, default_value_t = false)]
    pub t: bool,

    /// Specifies where to write the pidfile
    #[clap(long, value_parser, default_value_t=String::from("/var/run/bessd.pid"))]
    pub i: String,

    /// Run BESS in foreground mode (for developers)
    #[clap(short, value_parser, default_value_t = false)]
    pub f: bool,

    /// Kill existing BESS instance, if any
    #[clap(long, value_parser, default_value_t = false)]
    pub k: bool,

    /// Run BESS in debug mode (with debug log messages
    #[clap(long, value_parser, default_value_t = false)]
    pub d: bool,

    /// Skip checking that the process is running as root.
    #[clap(long, value_parser, default_value_t = false)]
    pub skip_root_check: bool,

    ///  Load modules from the specified directory
    #[clap(long, value_parser, default_value_t=format!("{}{}",bessd::get_current_directory(), "modules"))]
    pub modules: String,

    /// Generate a core dump on fatal faults
    #[clap(long, value_parser, default_value_t = false)]
    pub core_dump: bool,

    /// Disable the generation of a crash log file
    #[clap(long, value_parser, default_value_t = false)]
    pub no_crashlog: bool,

    // Note: currently BESS-managed hugepages do not support VFIO driver,
    //       so DPDK is default for now.
    /// Let DPDK manage hugepages
    #[clap(long, value_parser, default_value_t = true)]
    pub dpdk: bool,

    /// DPDK IOVA mode: pa or va. Set auto if not specifie
    #[clap(long, value_parser, default_value_t = String::from(""))]
    pub iova: String,

    /// Core ID for the default worker thread
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub c: u32,

    /// Specifies the URL where the BESS gRPC server should listen.
    /// If non empty, overrides -b and -p options.
    #[clap(long, value_parser, default_value_t = String::from(""))]
    pub grpc_url: String,

    /// Specifies the IP address of the interface the BESS gRPC server
    /// should bind to, if --grpc_url is empty. Deprecated, please use
    /// --grpc_url instead."
    #[clap(short, long, value_parser, default_value_t = K_DEFAULT_BIND_ADDR.to_string())]
    pub b: String,

    /// Specifies the TCP port on which BESS listens for controller connections,
    /// if --grpc_url is empty. Deprecated, please use --grpc_url instead
    #[clap(short, long, value_parser, default_value_t = K_DEFAULT_PORT)]
    pub p: u32,

    ///  Specifies per-socket hugepages to allocate (in MBs).
    ///  If set to 0, no hugepage is used
    #[clap(short, long, value_parser, default_value_t = 1024)]
    pub m: u32,

    ///  Specifies how many packet buffers to allocate per socket,
    ///  must be a power of 2.
    #[clap(long, value_parser, default_value_t = 262144)]
    pub buffers: u32,
}

pub fn validate_iova_mode(value: &str) -> bool {
    (value == "") || (value == "pa") || (value == "va")
}

// static bool _iova_dummy[[maybe_unused]] =
//     google::RegisterFlagValidator(&FLAGS_iova, &ValidateIovaMode);

// pub fn validate_core_id(value: u32) -> bool {
//     if !is_cpu_present(value) {
//         error!("Invalid core ID: {}", value);
//         false
//     }
//     true
// }

// static const bool _c_dummy[[maybe_unused]] = google::RegisterFlagValidator(&FLAGS_c, &ValidateCoreID);
pub fn validate_tcp_port(value: u32) -> bool {
    if value <= 0 {
        error!("Invalid TCP port number: {}", value);
        return false;
    }
    true
}

// static const bool _p_dummy[[maybe_unused]] =
//     google::RegisterFlagValidator(&FLAGS_p, &ValidateTCPPort);

pub fn validate_megabytes_per_socket(value: i32) -> bool {
    if value < 0 {
        error!("Invalid memory size: {}", value);
        return false;
    }
    true
}

// static const bool _m_dummy[[maybe_unused]] =
//     google::RegisterFlagValidator(&FLAGS_m, &ValidateMegabytesPerSocket);

pub fn validate_buffers_per_socket(value: u32) -> bool {
    if value <= 0 {
        error!("Invalid number of buffers: {}", value);
        return false;
    }

    if (value & (value - 1)) > 0 {
        error!("Number of buffers must be a power of 2: {}", value);
        return false;
    }
    true
}

// static const bool _buffers_dummy[[maybe_unused]] =
// google::RegisterFlagValidator(&FLAGS_buffers, &ValidateBuffersPerSocket);
