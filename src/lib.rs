//! Shadowsocks service command line utilities

use std::process::ExitCode;
use clap::Command;

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
pub unsafe extern "C" fn main_run_sslocal_for_ios(os: Vec<&str>) -> ExitCode {
    let mut app = Command::new("shadowsocks")
        .version(VERSION)
        .about("A fast tunnel proxy that helps you bypass firewalls. (https://shadowsocks.org)");
    app = local::define_command_line_options(app);
    let matches = app.get_matches_from(os);
    local::main(&matches)
}