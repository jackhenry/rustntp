use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

use rustntp::packet::NTPPacket;
use tokio::net::UdpSocket;

use crate::timeprovider::TimeProviderCache;

pub struct ClientModeHandler<'a> {
    socket: &'a UdpSocket,
    peer: &'a SocketAddr,
    client_message: &'a [u8],
    provider_cache: &'a Arc<Mutex<TimeProviderCache>>,
}

impl<'a> ClientModeHandler<'a> {
    pub fn new(
        socket: &'a UdpSocket,
        peer: &'a SocketAddr,
        client_message: &'a [u8],
        provider_cache: &'a Arc<Mutex<TimeProviderCache>>,
    ) -> Self {
        Self {
            socket,
            peer,
            client_message,
            provider_cache,
        }
    }

    pub async fn process(&self) {
        let mut packet = NTPPacket::from(self.client_message);
        packet.mark_received();
        let provider = &self.provider_cache;
        let cache = self.provider_cache.lock().unwrap();
        let builder = NTPPacket::builder()
            .leap(cache.leap_indicator)
            .version(4)
            .mode(4)
            .stratum(cache.startum)
            .poll(17)
            .precision(cache.precision)
            .root_delay(cache.root_delay)
            .root_dispersion(cache.root_dispersion)
            .ref_id(cache.ref_id)
            .reference(cache.last_sync_timestamp)
            .originate(packet.originate)
            .receive(packet.receive)
            .transmit(packet.transmit);
        // drop the lock so other threads can use the cache
        drop(cache);
        let mut server_packet = builder.build();
        server_packet.mark_for_transmission();
        if let Ok(_response) = self
            .socket
            .send_to(&server_packet.to_network_bytes(), self.peer)
            .await
        {
            tracing::debug!("Sent response to client");
        } else {
            tracing::debug!("Unable to send response to cient");
        }
    }
}
