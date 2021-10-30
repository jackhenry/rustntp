use std::net::{IpAddr, TcpStream};

use argh::FromArgs;

#[derive(FromArgs)]
/// Client arguments
struct ClientArguments {
    #[argh(positional)]
    host: String,

    #[argh(positional)]
    port: u8,
}

fn main() -> std::io::Result<()> {
    let options: ClientArguments = argh::from_env();

    let ipaddr = match options.host.parse::<IpAddr>() {
        Ok(ipaddr) => ipaddr,
        Err(_) => panic!("Invalid ip address provided."),
    };

    let port = options.port;
    let url = format!("{}:{}", ipaddr, port);
    let mut stream = TcpStream::connect(&url)?;

    Ok(())
}
