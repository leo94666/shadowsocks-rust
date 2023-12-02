//! Shadowsocks service command line utilities

use std::ffi::{CStr, CString};
use std::io::BufRead;
use std::os::raw::c_char;
use clap::{arg, Command};
use libc::{c_int, printf};

pub mod allocator;
pub mod config;
#[cfg(unix)]
pub mod daemonize;
#[cfg(feature = "logging")]
pub mod logging;
pub mod monitor;
pub mod password;
pub mod service;
pub mod sys;
pub mod vparser;

/// Exit code when server exits unexpectedly
pub const EXIT_CODE_SERVER_EXIT_UNEXPECTEDLY: sysexits::ExitCode = sysexits::ExitCode::Software;
/// Exit code when server aborted
pub const EXIT_CODE_SERVER_ABORTED: sysexits::ExitCode = sysexits::ExitCode::Software;
/// Exit code when loading configuration from file fails
pub const EXIT_CODE_LOAD_CONFIG_FAILURE: sysexits::ExitCode = sysexits::ExitCode::Config;
/// Exit code when loading ACL from file fails
pub const EXIT_CODE_LOAD_ACL_FAILURE: sysexits::ExitCode = sysexits::ExitCode::Config;
/// Exit code when insufficient params are passed via CLI
pub const EXIT_CODE_INSUFFICIENT_PARAMS: sysexits::ExitCode = sysexits::ExitCode::Usage;

/// Build timestamp in UTC
pub const BUILD_TIME: &str = build_time::build_time_utc!();

/// shadowsocks version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


use service::local;

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn sslocal_for_ios(os: *mut c_char, count: c_int) {

    let cmd = CString::from_raw(os.wrapping_add(0));
    println!("cmd,{:?}", cmd);


    let args: Vec<&str> = cmd.to_str().unwrap().split(" ").collect();

    let mut app = Command::new("shadowsocks")
        .version(VERSION)
        .about("A fast tunnel proxy that helps you bypass firewalls. (https://shadowsocks.org)");
    app = local::define_command_line_options(app);

    //let args = args.iter().map(|v| v.to_str().unwrap());

    let matches = app.get_matches_from(args);
    local::main(&matches);
}