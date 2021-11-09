use std::net::SocketAddr;

use rustntp::packet::NTPPacket;
use tokio::net::UdpSocket;

use crate::timeprovider::TimeProvider;

pub struct ClientModeHandler<'a, T>
where
    T: TimeProvider,
{
    socket: &'a UdpSocket,
    peer: &'a SocketAddr,
    client_message: &'a [u8],
    provider: &'a T,
}

impl<'a, T> ClientModeHandler<'a, T>
where
    T: TimeProvider,
{
    pub fn new(
        socket: &'a UdpSocket,
        peer: &'a SocketAddr,
        client_message: &'a [u8],
        provider: &'a T,
    ) -> Self {
        Self {
            socket,
            peer,
            client_message,
            provider,
        }
    }

    pub async fn process(&self) {
        let mut packet = NTPPacket::from(self.client_message);
        packet.mark_received(self.provider.get_ntp64_timestamp());
        let provider = &self.provider;
        let builder = NTPPacket::builder()
            .leap(provider.get_leap_indicator())
            .version(4)
            .mode(4)
            .stratum(provider.get_stratum())
            .poll(17)
            .precision(provider.get_precision())
            .root_delay(provider.get_root_delay())
            .root_dispersion(provider.get_root_dispersion())
            .ref_id(Some(provider.get_ref_id()))
            .reference(packet.reference)
            .originate(packet.originate)
            .receive(packet.receive)
            .transmit(packet.transmit);
        let mut server_packet = builder.build();
        server_packet.mark_for_transmission(self.provider.get_ntp64_timestamp());
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
