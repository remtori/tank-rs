#![feature(drain_filter)]

use std::net::TcpListener;

use anyhow::Context;
use clap::{App, AppSettings, Arg};

mod server;
mod timer;

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("GameServer")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("id")
                .takes_value(true)
                .required(true)
                .help("Game server id, default is an random u64"),
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .short("h")
                .takes_value(true)
                .help("Set listening ip, default is 0.0.0.0")
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .required(true)
                .help("Set listening port"),
        )
        .arg(
            Arg::with_name("tps")
                .long("tps")
                .short("t")
                .takes_value(true)
                .help("Server tick rate / Tick Per Seconds, default is 128")
                .default_value("128"),
        )
        .get_matches_safe()
        .unwrap_or_else(|err| {
            // Make sure all error printed to stderr
            eprintln!("{}", err.message);
            std::process::exit(1);
        });

    let id = matches.value_of("id").unwrap();
    let host = matches.value_of("host").unwrap();
    let port = matches
        .value_of("port")
        .unwrap()
        .parse::<u16>()
        .context("port must be u16")?;
    let tps = matches
        .value_of("tps")
        .unwrap()
        .parse::<u16>()
        .context("tps must be u16")?;

    let listener = TcpListener::bind((host, port)).context(format!("Failed to listen to: {}:{}", host, port))?;
    listener
        .set_nonblocking(true)
        .context("Failed to set TcpListener to nonblocking")?;

    server::Server::new(id.to_owned(), tps as u32, listener).run();
}
