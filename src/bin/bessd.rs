use bess::bessd;
use bess::opts::*;
// use bess::bessctl;
use clap::Parser;
use log::*;

#[tokio::main]
async fn main() {
    let flags = Options::parse();
    // FLAGS_logbuflevel = -1;
    // FLAGS_colorlogtostderr = true;
    // google::InitGoogleLogging(argv[0]);
    // google::InstallFailureFunction(bess::debug::GoPanic);
    // debug::SetTrapHandler();

    // google::SetVersionString(VERSION);
    // google::SetUsageMessage("BESS Command Line Options:");
    // google::ParseCommandLineFlags(&argc, &argv, true);
    // let pidfile_fd = bessd::CheckUniqueInstance(flag.i);
    // ignore_result(bessd::SetResourceLimit());

    let _signal_fd = -1;
    if flags.f {
        info!("Launching BESS daemon in process mode...");
    } else {
        info!("Launching BESS daemon in background...");
        bessd::daemonize();
        // if flags.logtostderr == true || flags.alsologtostderr == true {
        //   flags.logtostderr = false;
        //   flags.alsologtostderr = false;
        //   warn!("Daemon doesn't get attached to stdio. -logtostderr and -alsologtostderr options are ignored");
        // }
        // signal_fd = bessd::daemonize();
    }

    bessd::process_command_line_args();
    bessd::check_running_as_root();

    info!("bessd {}", option_env!("CARGO_PKG_VERSION").unwrap());

    // Store our PID (child's, if daemonized) in the PID file.
    // bessd::write_pid_file(pidfile_fd, getpid());

    // Load plugins
    let dir = flags.modules.as_str();
    if !bessd::load_plugins(dir) {
        warn!(
            "load_plugins() failed to load from directory: {}",
            dir.clone()
        );
    };
    info!("Plugin loaded from directory: {}", dir.clone());
    // TODO
    // bess::PacketPool::CreateDefaultPools(FLAGS_buffers);

    // PortBuilder::InitDrivers();

    // let server = bessctl:: ApiServer::new();
    let mut grpc_url = flags.grpc_url;
    if grpc_url.is_empty() {
        grpc_url = format!("{}:{}", flags.b, flags.p);
    }

    // server.listen(&grpc_url);

    // Signal the parent that all initialization has been finished.
    // if (!flags.f) {
    //   let one: u64 = 1;
    //   if (write(signal_fd, &one, sizeof(one)) < 0) {
    //     error!("write(signal_fd)");
    //   }
    //   close(signal_fd);
    // }

    // server.run();

    // TODO: DPDK required
    // rte_eal_mp_wait_lcore();

    info!("BESS daemon has been gracefully shut down");

    // return 0;
}
