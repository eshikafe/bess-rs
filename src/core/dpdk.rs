#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::memory;
use crate::opts;
use crate::worker;
use log;

// #include <rte_config.h>
// #include <rte_cycles.h>
// #include <rte_eal.h>
// #include <rte_ethdev.h>


pub fn is_dpdk_initialized() -> bool {
    todo!()
}

// Initialize DPDK, with the specified amount of hugepage memory.
// Safe to call multiple times.
pub fn init_dpdk(dpdk_mb_per_socket: i32) {
    todo!();
}

// void disable_syslog() {
//   setlogmask(0x01);
// }

// void enable_syslog() {
//   setlogmask(0xff);
// }


// class CmdLineOpts {
//  public:
//   explicit CmdLineOpts(std::initializer_list<std::string> args)
//       : args_(), argv_({nullptr}) {
//     Append(args);
//   }

//   void Append(std::initializer_list<std::string> args) {
//     for (const std::string &arg : args) {
//       args_.emplace_back(arg.begin(), arg.end());
//       args_.back().push_back('\0');
//       argv_.insert(argv_.begin() + argv_.size() - 1, args_.back().data());
//     }
//   }

//   char **Argv() { return argv_.data(); }

//   int Argc() const { return args_.size(); }

//   std::string Dump() {
//     std::ostringstream os;
//     os << "[";
//     for (size_t i = 0; i < args_.size(); i++) {
//       os << (i == 0 ? "" : ", ") << '"' << args_[i].data() << '"';
//     }
//     os << "]";
//     return os.str();
//   }

//  private:
//   // Contains a copy of each argument.
//   std::vector<std::vector<char>> args_;
//   // Pointer to each argument (in `args_`), plus an extra `nullptr`.
//   std::vector<char *> argv_;
// };

// void init_eal(int dpdk_mb_per_socket, std::string nonworker_corelist) {
//   CmdLineOpts rte_args{
//       "bessd",
//       "--master-lcore",
//       std::to_string(RTE_MAX_LCORE - 1),
//       "--lcore",
//       std::to_string(RTE_MAX_LCORE - 1) + "@" + nonworker_corelist,
//       // Do not bother with /var/run/.rte_config and .rte_hugepage_info,
//       // since we don't want to interfere with other DPDK applications.
//       "--no-shconf",
//       // TODO(sangjin) switch to dynamic memory mode
//       "--legacy-mem",
//   };

//   if (dpdk_mb_per_socket <= 0) {
//     rte_args.Append({"--iova", (FLAGS_iova != "") ? FLAGS_iova : "va"});
//     rte_args.Append({"--no-huge"});

//     // even if we opt out of using hugepages, many DPDK libraries still rely on
//     // rte_malloc (e.g., rte_lpm), so we need to reserve some (normal page)
//     // memory in advance. We allocate 512MB (this is shared among nodes).
//     rte_args.Append({"-m", "512"});
//   } else {
//     rte_args.Append({"--iova", (FLAGS_iova != "") ? FLAGS_iova : "pa"});

//     std::string opt_socket_mem = std::to_string(dpdk_mb_per_socket);
//     for (int i = 1; i < NumNumaNodes(); i++) {
//       opt_socket_mem += "," + std::to_string(dpdk_mb_per_socket);
//     }

//     rte_args.Append({"--socket-mem", opt_socket_mem});

//     // Unlink mapped hugepage files so that memory can be reclaimed as soon as
//     // bessd terminates.
//     rte_args.Append({"--huge-unlink"});
//   }

//   // reset getopt()
//   optind = 0;

//   // DPDK creates duplicated outputs (stdout and syslog).
//   // We temporarily disable syslog, then set our log handler
//   cookie_io_functions_t dpdk_log_init_funcs;
//   cookie_io_functions_t dpdk_log_funcs;

//   std::memset(&dpdk_log_init_funcs, 0, sizeof(dpdk_log_init_funcs));
//   std::memset(&dpdk_log_funcs, 0, sizeof(dpdk_log_funcs));

//   dpdk_log_init_funcs.write = &dpdk_log_init_writer;
//   dpdk_log_funcs.write = &dpdk_log_writer;

//   FILE *org_stdout = stdout;
//   stdout = fopencookie(nullptr, "w", dpdk_log_init_funcs);

//   disable_syslog();
//   LOG(INFO) << "Initializing DPDK EAL with options: " << rte_args.Dump();
//   int ret = rte_eal_init(rte_args.Argc(), rte_args.Argv());
//   if (ret < 0) {
//     LOG(FATAL) << "rte_eal_init() failed: ret = " << ret
//                << " rte_errno = " << rte_errno << " ("
//                << rte_strerror(rte_errno) << ")";
//   }

//   enable_syslog();
//   fclose(stdout);
//   stdout = org_stdout;

//   rte_openlog_stream(fopencookie(nullptr, "w", dpdk_log_funcs));
// }

// // Returns the current affinity set of the process as a string,
// // in the "corelist" format (e.g., "0-12,16-28")
// std::string GetNonWorkerCoreList() {
//   std::string corelist;
//   cpu_set_t set;

//   int ret = pthread_getaffinity_np(pthread_self(), sizeof(set), &set);
//   if (ret < 0) {
//     PLOG(WARNING) << "pthread_getaffinity_np()";
//     return 0;  // Core 0 as a fallback
//   }

//   // Choose the last core available
//   for (int i = 0; i < CPU_SETSIZE; i++) {
//     if (CPU_ISSET(i, &set)) {
//       int start = i;
//       while (i < CPU_SETSIZE && CPU_ISSET(i, &set)) {
//         i++;
//       }
//       int end = i - 1;

//       std::string group = std::to_string(start);
//       if (start < end) {
//         group += "-" + std::to_string(end);
//       }

//       if (corelist == "") {
//         corelist += group;
//       } else {
//         corelist += "," + group;
//       }
//     }
//   }

//   if (corelist == "") {
//     // This should never happen, but just in case...
//     PLOG(WARNING) << "No core is allowed for the process?";
//     corelist = "0";
//   }

//   return corelist;
// }

// bool is_initialized = false;

// }  // namespace

// bool IsDpdkInitialized() {
//   return is_initialized;
// }

// void InitDpdk(int dpdk_mb_per_socket) {
//   current_worker.SetNonWorker();

//   if (!is_initialized) {
//     is_initialized = true;
//     init_eal(dpdk_mb_per_socket, GetNonWorkerCoreList());
//   }
// }

// }  // namespace bess
