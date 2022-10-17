// Copyright (c) 2014-2016, The Regents of the University of California.
// Copyright (c) 2016-2017, Nefeli Networks, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
// list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// * Neither the names of the copyright holders nor the names of their
// contributors may be used to endorse or promote products derived from this
// software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

// #include <dirent.h>
// #include <dlfcn.h>
// #include <sys/file.h>
// #include <sys/resource.h>
// #include <sys/stat.h>
// #include <unistd.h>

// #include <glog/logging.h>

// #include <algorithm>
// #include <cerrno>
// #include <csignal>
// #include <cstdint>
// #include <cstdio>
// #include <cstdlib>
// #include <iostream>
// #include <list>
// #include <string>
// #include <tuple>


// #include "debug.h"
use crate::core::debug;
use crate::core::opts::*;
use clap::Parser;
use exitcode;
use log::*;
use env_logger::{Builder, WriteStyle, Target};
use std::io::Write;
use chrono;
use env_logger::fmt::Color;
use std::process::exit;

use libc::*;
use nix::*;
// #include "port.h"

// How log messages are processed in BESS?
// - In daemon mode:
//   - via Google glog (recommended)
//     - LOG(*) -> /tmp/bessd.*
//     - Note that any log messages are routed to stderr, regardless of glog
//       command-line flags, such as "stderrthreshold"
//   - via libc stdout/stderr: e.g., printf(...) , fprintf(stderr, ...)
//     - stdout -> stdout_funcs -> LOG(INFO) -> /tmp/bessd.INFO
//     - stderr -> stderr_funcs -> LOG(WARNING) -> /tmp/bessd.[INFO|WARNING]
//   - via libstdc++ cout/cerr
//     - cout -> stdout_buf -> LOG(INFO) -> /tmp/bessd.INFO
//     - cerr -> stderr_buf -> LOG(WARNING) -> /tmp/bessd.INFO
// - In process mode (foreground; -f option):
//   - via Google glog (recommended)
//     - LOG(*) -> standard error (colored, if applicable)
//   - via libc/libstdc++
//     - stdout/cout -> standard output
//     - stderr/cerr -> standard error (colored, currently always)

// Utility routines for the main bess daemon.

// When Modules extend other Modules, they may reference a shared object
// that has not yet been loaded by the BESS daemon. kInheritanceLimit is
// the number of passes that will be made while loading Module shared objects,
// and thus the maximum inheritance depth of any Module.
pub const K_INHERITANCE_LIMIT: u32 = 10;

// namespace {

// // Intercepts all output messages to an ostream-based class and redirect them
// // to glog, with a specified log severity level. This behavior lasts as long as
// // the object is alive, and afterwards the original behavior is restored.
// class StreambufLogger : public std::streambuf {
//  public:
//   StreambufLogger(std::ostream &stream, google::LogSeverity severity)
//       : stream_(stream), log_level_(severity) {
//     org_streambuf_ = stream_.rdbuf(this);
//   }

//   // Restores the original streambuf
//   virtual ~StreambufLogger() { stream_.rdbuf(org_streambuf_); }

//   // Redirects all << operands to glog
//   std::streamsize xsputn(const char_type *s, std::streamsize count) {
//     WriteToGlog(s, count);
//     return count;
//   }

//   // NOTE: This function is never going to be called for std::cout and
//   // std::cerr, but implemented for general streams, for completeness)
//   int_type overflow(int_type v) {
//     char_type c = traits_type::to_char_type(v);
//     WriteToGlog(&c, 1);
//     return traits_type::not_eof(v);
//   }

//  private:
//   void WriteToGlog(const char_type *s, std::streamsize count) {
//     // prevent glog from creating an empty line even with no message
//     if (count <= 0) {
//       return;
//     }

//     // same as above. ignore << std::endl
//     if (count == 1 && s[0] == '\n') {
//       return;
//     }

//     // ignore trailing '\n', since glog will append it automatically
//     if (s[count - 1] == '\n') {
//       count--;
//     }

//     // since this is not an macro, we do not have __FILE__, and __LINE__ of
//     // the caller.
//     google::LogMessage("<unknown>", 0, log_level_).stream()
//         << std::string(s, count);
//   }

//   std::ostream &stream_;
//   std::streambuf *org_streambuf_;
//   google::LogSeverity log_level_;
// };

// }  // namespace

// Process command line arguments from gflags.
pub fn process_command_line_args() {
    let flags = Options::parse();
    if flags.t {
        debug::dump_types();
        exit(exitcode::OK);
        //   unsafe {exit(EXIT_SUCCESS)};
    }

    if flags.f {
        // google::LogToStderr();
        env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.target(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.level(),
                record.args()
            )
        })
        .write_style(WriteStyle::Always )
        .target(Target::Stderr)
        .init();
    }
}

// Checks that we are running as superuser.
pub fn check_running_as_root() {
    let flag = Options::parse();
    if !flag.skip_root_check {
        let euid = unistd::geteuid();
        if !unistd::Uid::is_root(euid) {
            error!("You need root privilege to run the BESS daemon");
            exit(exitcode::TEMPFAIL);
        }
    }
    // Great power comes with great responsibility.
    unsafe { umask(S_IWGRP | S_IWOTH) };
}

// Write the pid value to the given file fd.  Overwrites anything present at
// that fd.  Dies if unable to overwrite the file.
pub fn write_pid_file(fd: u32, pid: u32) {}

// Read the pid value from the given file fd.  Returns true and the read pid
// value upon success.  Returns false upon failure.
pub fn read_pid_file(fd: u32) -> (bool, u32) {
    (false, 0)
}

// Tries to acquire the daemon pidfile lock for the file open at the given fd.
// Dies if an error occurs when trying to acquire the lock.  Returns a pair
// <lockheld, pid> where lockheld is a bool indicating if the lock is held and
// pid is a pid_t that is non-zero if lockheld is true indicating the process
// holding the lock.
pub fn try_acquire_pid_file_lock(fd: u32) -> (bool, u32) {
    (false, 0)
}

// Ensures that we are a unique instance.
// Returns the (locked) file descriptor of pidfile_path.
pub fn check_unique_instance(pidfile_path: &str) -> u32 {
    0
}

// Starts BESS as a daemon running in the background.
pub fn daemonize() -> i32 {
    0
}

// Sets BESS's resource limit.  Returns true upon success.
pub fn set_resource_limit() -> bool {
    true
}

// Load an indiviual plugin specified by path. Return true upon success.
pub fn load_plugin(path: String) -> bool {
    true
}

// Unload a loaded plugin specified by path. Return true upon success.
pub fn unload_plugin(path: &str) -> bool {
    true
}

// Load all the .so files in the specified directory. Return true upon success.
pub fn load_plugins(directory: &str) -> bool {
    true
}

// List all imported .so files.
pub fn list_plugins() -> Vec<String> {
    vec!["".to_string()]
}

// Return the current executable's own directory. For example, if the location
// of the executable is /opt/bess/core/bessd, returns /opt/bess/core/ (with the
// slash at the end).
pub fn get_current_directory() -> String {
    "".to_string()
}
