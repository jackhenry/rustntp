use std::{net::IpAddr, panic};

use argh::FromArgs;
use config::ClientConfig;
use rustntp::tcp::NTSPacketTransform;

use crate::{builder::ClientEstablishmentBuilder, config::ConfigManager};

mod builder;
mod config;
mod connection;
mod execution;
mod ntske;
mod response;

#[derive(FromArgs)]
/// Client arguments
struct ClientArguments {
    #[argh(positional)]
    host: String,

    #[argh(positional)]
    port: u16,
}

fn main() -> std::io::Result<()> {
    //let options: ClientArguments = argh::from_env();
    let config = ConfigManager::load_from_or_default(&String::from("/home/jack/"));
    println!("{:?}", config);
    Ok(())
}
