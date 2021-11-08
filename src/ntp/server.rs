use std::net::SocketAddr;

use tokio::net::UdpSocket;

pub struct Server {
    pub socket: UdpSocket,
    pub buffer: Vec<u8>,
    pub to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    pub async fn run(self) -> Result<(), std::io::Error> {
        let Server {
            socket,
            mut buffer,
            mut to_send,
        } = self;

        loop {
            if let Some((size, peer)) = to_send {
                let message = &buffer[..size];
                tracing::debug!("{:?} size: {}", message, size);
                let amt = socket.send_to(&buffer[..size], &peer).await?;

                tracing::debug!("Echoed {}/{} bytes to {}", amt, size, peer);
            }

            to_send = Some(socket.recv_from(&mut buffer).await?);
        }
    }
}
