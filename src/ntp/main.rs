use argh::FromArgs;
use std::error::Error;
use std::net::ToSocketAddrs;
use tokio::net::UdpSocket;

use crate::server::Server;

mod handler;
mod server;
mod timeprovider;

/// ntp command line options
#[derive(FromArgs)]
struct Options {
    /// bind addr
    #[argh(positional)]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::FULL)
        .init();

    let options: Options = argh::from_env();
    let addr = options
        .addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))?;

    println!("Address: {:?}", addr);
    let socket = UdpSocket::bind(&addr).await?;
    tracing::debug!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buffer: vec![0; 1024],
        to_send: None,
    };

    server.run().await?;

    Ok(())
}
