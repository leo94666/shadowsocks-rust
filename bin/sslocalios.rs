//! This is a binary running in the local environment
//!
//! You have to provide all needed configuration attributes via command line parameters,
//! or you could specify a configuration file. The format of configuration file is defined
//! in mod `config`.

use std::env;
use std::env::ArgsOs;
use std::process::ExitCode;

use clap::Command;
use shadowsocks_rust::service::local;

fn main() -> ExitCode {
    let arg_vec = vec!["sslocalios","-c", "D:\\li\\rust\\shadowsocks-rust\\run\\client-config.json"];
    run_sslocal_for_ios(arg_vec)
}

fn run_sslocal_for_ios(os: Vec<&str>) -> ExitCode {
    let mut app = Command::new("shadowsocks")
        .version(shadowsocks_rust::VERSION)
        .about("A fast tunnel proxy that helps you bypass firewalls. (https://shadowsocks.org)");
    app = local::define_command_line_options(app);
    let matches = app.get_matches_from(os);
    local::main(&matches)
}
