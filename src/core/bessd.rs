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

use crate::core::debug;
use crate::core::opts::*;
use chrono;
use clap::Parser;
use env_logger::fmt::Color;
use env_logger::{Builder, Target, WriteStyle};
use exitcode;
use log::*;
use std::io::Write;
use std::process::exit;

use libc::*;
use nix::*;
// #include "port.h"

// Utility routines for the main bess daemon.

// When Modules extend other Modules, they may reference a shared object
// that has not yet been loaded by the BESS daemon. kInheritanceLimit is
// the number of passes that will be made while loading Module shared objects,
// and thus the maximum inheritance depth of any Module.
pub const K_INHERITANCE_LIMIT: u32 = 10;

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
                let mut style = buf.style();
                match record.level() {
                    Level::Error | Level::Warn => style.set_color(Color::Red),
                    Level::Debug => style.set_color(Color::Blue),
                    Level::Trace => style.set_color(Color::Magenta),
                    Level::Info => style.set_color(Color::Green),
                };

                writeln!(
                    buf,
                    "[{} {} {}:{}] [{}] - {}",
                    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    style.value(record.target()),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    style.value(record.level()),
                    record.args()
                )
            })
            .filter(None, LevelFilter::Debug)
            .write_style(WriteStyle::Always)
            .target(Target::Stdout)
            .init();
    // env_logger::init();
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
