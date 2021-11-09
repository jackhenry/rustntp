use std::net::SocketAddr;

use rustntp::packet::NTPPacket;
use rustntp::protocol::ntp::Timestamp;
use rustntp::systime::SystemTime;
use tokio::net::UdpSocket;

use crate::handler::ClientModeHandler;
use crate::timeprovider::LoopbackProvider;
use crate::timeprovider::TimeProvider;

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

        let time_provider = LoopbackProvider::new();

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
                let client_handler = ClientModeHandler::<LoopbackProvider>::new(
                    &socket,
                    &peer,
                    &message,
                    &time_provider,
                );
                client_handler.process().await;

                /*                 tracing::debug!("{:?}", packet);
                if let Ok(num_bytes) = socket
                    .send_to(&packet.to_network_bytes().to_vec(), &peer)
                    .await
                {
                    tracing::debug!("Echoed {}/{} bytes to {}", num_bytes, size, peer);
                } else {
                    tracing::debug!("Unable to send response to client {}", peer.ip());
                } */
            };
        }
    }
}
