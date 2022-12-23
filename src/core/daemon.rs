
use std::{fs::File};
use daemonize::Daemonize;

const STD_OUT_FILE_PATH:&str =  "/tmp/bessd.out";
const STD_ERR_FILE_PATH:&str = "/tmp/bessd.err";
const PID_FILE_PATH:&str = "/tmp/bessd.pid";

pub fn start_daemon(){

    let std_out = File::create(STD_OUT_FILE_PATH).unwrap();
    let std_err = File::create(STD_ERR_FILE_PATH).unwrap();
    let daemonize = Daemonize::new()
        .pid_file(PID_FILE_PATH) // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .group("daemon") // Group name
        .user("unknown")
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(std_out)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(std_err)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, started daemon successfully"),
        Err(e) => eprintln!("Error, {}", e),
        }
}