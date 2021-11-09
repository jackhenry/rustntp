use std::net::SocketAddr;

use rustntp::packet::NTPPacket;
use tokio::net::UdpSocket;

pub struct Server {
    pub socket: UdpSocket,
    pub buffer: Vec<u8>,
    pub to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    pub async fn run(self) -> Result<(), rustntp::Error> {
        let Server {
            socket,
            mut buffer,
            mut to_send,
        } = self;

        loop {
            // Wait for next packet to receive
            tracing::debug!("waiting for next packet");
            to_send = match socket.recv_from(&mut buffer).await {
                Ok(message) => Some(message),
                Err(error) => {
                    tracing::debug!("Read error {:?}", error);
                    None
                }
            };

            if let Some((size, peer)) = to_send {
                // NTP packet should be at least 48 bytes
                if size < 48 {
                    tracing::debug!(
                        "Received a packet that didn't reach minimum length of 48 bytes."
                    );
                    continue;
                }
                let message = &buffer[..size];
                println!("{:?}", message);
                let packet = NTPPacket::from(message);
                println!("{:?}", packet);
                if let Ok(num_bytes) = socket.send_to(message, &peer).await {
                    tracing::debug!("Echoed {}/{} bytes to {}", num_bytes, size, peer);
                } else {
                    tracing::debug!("Unable to send response to client {}", peer.ip());
                }
            };
        }
    }
}
