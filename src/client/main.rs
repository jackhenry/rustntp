use std::{net::IpAddr, panic};

use argh::FromArgs;
use config::ClientConfig;
use rustntp::tcp::NTSPacketTransform;

use crate::{config::{ConfigManager, NTSKeyExchangeServer}, execution::{ExecutionCreator, ExecutionHandler, ExecutionRequest}, ntske::KeyEstablishment};

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
    let config = ConfigManager::load_from_or_default(&String::from("/home/jack/")).unwrap();
    println!("{:?}", config);

    let server = NTSKeyExchangeServer {
        address: Some(String::from("127.0.0.1")),
        port: Some(4406)
    };
    
    let mut handler = ExecutionHandler::from(&config);
    handler.enqueue(ExecutionRequest { handler: KeyEstablishment::create_executable() });
    println!("{:?}", handler.execute_all());

    Ok(())
}
