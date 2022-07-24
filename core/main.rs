mod opts;
mod bessd;
mod debug;
mod port;
mod dpdk;
mod scheduler;
mod packet_pool;
mod traffic_class;
mod clap::Parser;
mod log::*;

// #include <rte_launch.h>
// dpdk crate

// mod bess::bessctl;
// mod bess::bessd;
// mod bess::debug;
// mod bess::opts;
// use bess::packet_pool;
// use bess::port;
// #include "utils/format.h"
// mod utils;

fn main() {
  FLAGS_logbuflevel = -1;
  FLAGS_colorlogtostderr = true;
  // google::InitGoogleLogging(argv[0]);
  env_logger::init();

  google::InstallFailureFunction(bess::debug::GoPanic);
  debug::SetTrapHandler();

  google::SetVersionString(VERSION);
  google::SetUsageMessage("BESS Command Line Options:");
  google::ParseCommandLineFlags(&argc, &argv, true);
  bessd::process_command_line_args(); 
  bessd::check_running_as_root();

  int pidfile_fd = bess::bessd::CheckUniqueInstance(FLAGS_i);
  ignore_result(bess::bessd::SetResourceLimit());

  int signal_fd = -1;
  if (FLAGS_f) {
    LOG(INFO) << "Launching BESS daemon in process mode...";
  } else {
    LOG(INFO) << "Launching BESS daemon in background...";

    if (FLAGS_logtostderr == true || FLAGS_alsologtostderr == true) {
      FLAGS_logtostderr = false;
      FLAGS_alsologtostderr = false;
      LOG(WARNING) << "Daemon doesn't get attached to stdio. "
                      "-logtostderr and -alsologtostderr options are ignored";
    }
    signal_fd = bessd::Daemonize();
  }

  LOG(INFO) << "bessd " << google::VersionString();

  // Store our PID (child's, if daemonized) in the PID file.
  bess::bessd::WritePidfile(pidfile_fd, getpid());

  // Load plugins
  if (!bess::bessd::LoadPlugins(FLAGS_modules)) {
    PLOG(WARNING) << "LoadPlugins() failed to load from directory: "
                  << FLAGS_modules;
  }

  bess::PacketPool::CreateDefaultPools(FLAGS_buffers);

  PortBuilder::InitDrivers();

  {
    ApiServer server;
    std::string grpc_url = FLAGS_grpc_url;
    if (grpc_url.empty()) {
      grpc_url = bess::utils::Format("%s:%d", FLAGS_b.c_str(), FLAGS_p);
    }

    server.Listen(grpc_url);

    // Signal the parent that all initialization has been finished.
    if (!FLAGS_f) {
      uint64_t one = 1;
      if (write(signal_fd, &one, sizeof(one)) < 0) {
        PLOG(FATAL) << "write(signal_fd)";
      }
      close(signal_fd);
    }

    server.Run();
  }

  rte_eal_mp_wait_lcore();

  LOG(INFO) << "BESS daemon has been gracefully shut down";

  return 0;
}
