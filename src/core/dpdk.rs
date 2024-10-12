#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::memory;
use crate::opts;
use crate::worker;
use clap::Parser;
use log;
use std::fmt;

use super::opts::Options;

// #include <rte_config.h>
// #include <rte_cycles.h>
// #include <rte_eal.h>
// #include <rte_ethdev.h>

struct DpdkState {
    is_initialized: bool,
}

impl DpdkState {
    fn new() -> Self {
        Self {
            is_initialized: false,
        }
    }

    // Check if DPDK is initialized
    fn is_dpdk_initialized(&self) -> bool {
        self.is_initialized
    }

    // Initialize DPDK, with the specified amount of hugepage memory.
    // Safe to call multiple times.
    fn init_dpdk(&mut self, dpdk_mb_per_socket: i32) {
        // current_worker.SetNonWorker();
        if !self.is_initialized {
            self.is_initialized = true;
            init_eal(dpdk_mb_per_socket, get_non_worker_core_list());
        }
    }
}
fn disable_syslog() {
    //setlogmask(0x01);
    todo!();
}

fn enable_syslog() {
    //setlogmask(0xff);
    todo!();
}

struct CmdLineOpts {
    args: Vec<String>,
}

impl CmdLineOpts {
    fn new(args: Vec<String>) -> Self {
        Self { args }
    }

    fn append(&mut self, args: Vec<String>) {
        self.args.extend(args);
    }

    fn argc(&self) -> usize {
        self.args.len()
    }

    fn argv(&self) -> Vec<&str> {
        self.args.iter().map(AsRef::as_ref).collect()
    }

    fn dump(&self) -> String {
        format!(
            "[{}]",
            self.args
                .iter()
                .map(|arg| format!("\"{}\"", arg))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl fmt::Display for CmdLineOpts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}

pub fn init_eal(dpdk_mb_per_socket: i32, nonworker_corelist: String) {
    let flags = Options::parse();
    let rte_args = CmdLineOpts::new(vec![
        "bessd".to_string(),
        "--master-lcore".to_string(),
        (RTE_MAX_LCORE - 1).to_string(),
        "--lcore".to_string(),
        format!("{}@{}", (RTE_MAX_LCORE - 1).to_string(), nonworker_corelist),
        // Do not bother with /var/run/.rte_config and .rte_hugepage_info,
        // since we don't want to interfere with other DPDK applications.
        "--no-shconf".to_string(),
        // TODO(sangjin) switch to dynamic memory mode
        "--legacy-mem".to_string(),
    ]);

    if dpdk_mb_per_socket <= 0 {
        // (FLAGS_iova != "") ? FLAGS_iova : "va"}
        let iova_arg = if !flags.iova.is_empty() {
            flags.iova.clone()
        } else {
            "va".to_string()
        };
        rte_args.append(vec!["--iova".to_string(), iova_arg]);

        rte_args.append(vec!["--no-huge".to_string()]);

        // even if we opt out of using hugepages, many DPDK libraries still rely on
        // rte_malloc (e.g., rte_lpm), so we need to reserve some (normal page)
        // memory in advance. We allocate 512MB (this is shared among nodes).
        rte_args.append(vec!["-m".to_string(), "512".to_string()]);
    } else {
        let iova_arg = if !flags.iova.is_empty() {
            flags.iova.clone()
        } else {
            "pa".to_string()
        };
        rte_args.append(vec!["--iova".to_string(), iova_arg]);

        let mut opt_socket_mem = dpdk_mb_per_socket.to_string();
        for i in 1..NumNumaNodes() {
            opt_socket_mem.push_str(&format!(",{}", dpdk_mb_per_socket));
        }

        rte_args.append(vec!["--socket-mem".to_string(), opt_socket_mem]);

        // Unlink mapped hugepage files so that memory can be reclaimed as soon as
        // bessd terminates.
        rte_args.append(vec!["--huge-unlink".to_string()]);
    }

    // reset getopt()
    //   optind = 0;

    // DPDK creates duplicated outputs (stdout and syslog).
    // We temporarily disable syslog, then set our log handler
    // cookie_io_functions_t dpdk_log_init_funcs;
    // cookie_io_functions_t dpdk_log_funcs;

    //   std::memset(&dpdk_log_init_funcs, 0, sizeof(dpdk_log_init_funcs));
    //   std::memset(&dpdk_log_funcs, 0, sizeof(dpdk_log_funcs));

    //   dpdk_log_init_funcs.write = &dpdk_log_init_writer;
    //   dpdk_log_funcs.write = &dpdk_log_writer;

    //   FILE *org_stdout = stdout;
    //   stdout = fopencookie(nullptr, "w", dpdk_log_init_funcs);

    disable_syslog();
    log::info!("Initializing DPDK EAL with options: {}", rte_args.dump());
    let ret = rte_eal_init(rte_args.argc(), rte_args.argv());
    if ret < 0 {
        log::error!("rte_eal_init() failed: ret = {}", ret)
        //   << "rte_eal_init() failed: ret = " << ret
        //    << " rte_errno = " << rte_errno << " ("
        //    << rte_strerror(rte_errno) << ")";
    }

    enable_syslog();
    //   fclose(stdout);
    //   stdout = org_stdout;

    rte_openlog_stream(fopencookie(nullptr, "w", dpdk_log_funcs));
}

// Returns the current affinity set of the process as a string,
// in the "corelist" format (e.g., "0-12,16-28")
fn get_non_worker_core_list() -> String {
    let mut corelist = String::from("");
    //   cpu_set_t set;

    //   int ret = pthread_getaffinity_np(pthread_self(), sizeof(set), &set);
    //   if (ret < 0) {
    //     PLOG(WARNING) << "pthread_getaffinity_np()";
    //     return 0;  // Core 0 as a fallback
    //   }

    // Choose the last core available
    for i in 0..CPU_SETSIZE {
        if (CPU_ISSET(i, &set)) {
            let start = i;
            while (i < CPU_SETSIZE && CPU_ISSET(i, &set)) {
                i += 1;
            }
            let end = i - 1;

            let mut group = start.to_string();
            if start < end {
                // group += "-" + std::to_string(end);
                group.push_str(&format!("-{}", end.to_string()));
            }

            if corelist.is_empty() {
                corelist.push_str(group.as_str());
            } else {
                corelist.push_str(&format!(",{}", group));
            }
        }
    }

    if corelist.is_empty() {
        // This should never happen, but just in case...
        log::warn!("No core is allowed for the process?");
        corelist = "0".to_string();
    }

    return corelist;
}
