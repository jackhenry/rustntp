use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio_rustls::TlsAcceptor;

pub struct ClientHandler {
    stream: TcpStream,
    acceptor: TlsAcceptor,
    peer_address: SocketAddr,
}

impl ClientHandler {
    pub fn from(stream: TcpStream, acceptor: TlsAcceptor, peer_address: SocketAddr) -> Self {
        Self {
            stream,
            acceptor,
            peer_address,
        }
    }

    pub fn run(&self) {}
}
