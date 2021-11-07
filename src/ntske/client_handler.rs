use std::array;
use std::io::Write;
use std::net::SocketAddr;
use std::thread;
use std::thread::Thread;
use std::time;
use std::time::Duration;
use std::time::SystemTime;

use tokio::io;
use tokio::io::split;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::runtime;
use tokio::spawn;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::TlsStream;

use crate::decoder::Decoder;

pub struct ClientHandler;

impl ClientHandler {
    pub async fn run(
        stream: TcpStream,
        acceptor: TlsAcceptor,
        peer_address: SocketAddr,
    ) -> io::Result<()> {
        let stream = acceptor.accept(stream).await?;
        tracing::trace!("Finished TLS handshake with {}", peer_address);
        let (mut reader, mut writer) = split(stream);

        // fetch stream
        let mut buffer: [u8; 1024] = [0; 1024];
        reader.read(&mut buffer).await?;
        // decode stream
        let decode_result = Decoder::decode_stream(&mut buffer);
        if let Ok(records) = decode_result {
            tracing::trace!("successfully decoded");
        } else {
            tracing::error!("{}", decode_result.unwrap_err());
        }

        Ok(())
    }
}
