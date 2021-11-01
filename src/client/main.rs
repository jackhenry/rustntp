use std::{net::IpAddr, panic};

use argh::FromArgs;

mod builder;
mod connection;

#[derive(FromArgs)]
/// Client arguments
struct ClientArguments {
    #[argh(positional)]
    host: String,

    #[argh(positional)]
    port: u16,
}

fn main() -> std::io::Result<()> {
    let builder = builder::EstablishmentBuilder::new();
    let establishment = builder.build();
    println!("{:?}", establishment);

    let options: ClientArguments = argh::from_env();

    let ipaddr = match options.host.parse::<IpAddr>() {
        Ok(ipaddr) => ipaddr,
        Err(_) => panic!("Invalid ip address provided."),
    };

    let port = options.port;

    let url = format!("{}:{}", ipaddr, port);
    println!("url: {}", url);

    Ok(())
}
