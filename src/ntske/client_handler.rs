use std::net::SocketAddr;
use std::thread;
use std::thread::Thread;
use std::time::Duration;

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::TlsAcceptor;

pub struct ClientHandler;

impl ClientHandler {
    pub async fn run(stream: TcpStream, acceptor: TlsAcceptor) -> io::Result<()> {
        let mut counter = 1;
        let mut stream = acceptor.accept(stream).await?;
        loop {
            let current_thread = thread::current();
            let name = current_thread.name().unwrap();
            println!("[{}]: {}", name, counter);
            stream.write_all(b"hello").await?;
            counter += 1;
            thread::sleep(Duration::from_secs(2));
        }
    }
}
