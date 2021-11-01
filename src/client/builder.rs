use std::net::{IpAddr, Ipv4Addr};

use rustntp::establishment::ClientEstablishment;
use rustntp::protocol::ntske::{
    ClientAEADAlgorithmRecord, EndOfMessageRecord, NextProtocolNegotiationRecord,
    PortNegotiationRecord, ServerNegotiationRecord,
};
use rustntp::protocol::NTP_PROTOCOL_ID;
use rustntp::support::{self, AES_SIV_CMAC_256};

pub struct EstablishmentBuilder {
    client_establishment: ClientEstablishment,
}

impl EstablishmentBuilder {
    pub fn new() -> EstablishmentBuilder {
        EstablishmentBuilder {
            client_establishment: ClientEstablishment::new(),
        }
    }

    pub fn end_of_message(&mut self) -> Result<&EstablishmentBuilder, ()> {
        self.client_establishment.end_of_message = Some(EndOfMessageRecord {});
        Ok(self)
    }

    pub fn next_protocol_negotiation(
        &mut self,
        protocol: u16,
    ) -> Result<&EstablishmentBuilder, rustntp::Error> {
        if matches!(protocol, support::NTP) {
            return Err(rustntp::Error::UnsupportedProtocol);
        }

        let protocol_ids = vec![NTP_PROTOCOL_ID];
        self.client_establishment.next_protocol_negotiation =
            Some(NextProtocolNegotiationRecord { protocol_ids });
        Ok(self)
    }

    pub fn aead_algorithm_negotiation(
        &mut self,
        algorithms: Vec<u16>,
    ) -> Result<&EstablishmentBuilder, rustntp::Error> {
        let invalid_algorithm = |algorithm| !support::SUPPORTED_AEAD_ALGORITHMS.contains(algorithm);

        if algorithms.iter().any(invalid_algorithm) {
            return Err(rustntp::Error::UnsupportedAlgorithm);
        }

        self.client_establishment.aead_algorithm_negotiation =
            Some(ClientAEADAlgorithmRecord { algorithms });
        Ok(self)
    }

    pub fn server_negotiation(
        &mut self,
        server_address: IpAddr,
    ) -> Result<&EstablishmentBuilder, ()> {
        self.client_establishment.server_negotiation =
            Some(ServerNegotiationRecord { server_address });
        Ok(self)
    }

    pub fn port_negotiation(&mut self, port: u16) -> Result<&EstablishmentBuilder, ()> {
        self.client_establishment.port_negotiation = Some(PortNegotiationRecord { port });
        Ok(self)
    }

    /// Initializes any unset ```ClientEstablishment``` fields before calling ```build()```
    pub fn build_with_defaults(&mut self) -> Result<&ClientEstablishment, rustntp::Error> {
        let mut establishment = &mut self.client_establishment;
        establishment
            .end_of_message
            .get_or_insert(EndOfMessageRecord {});

        establishment
            .next_protocol_negotiation
            .get_or_insert(NextProtocolNegotiationRecord {
                protocol_ids: vec![support::NTP],
            });

        establishment
            .aead_algorithm_negotiation
            .get_or_insert(ClientAEADAlgorithmRecord {
                algorithms: vec![AES_SIV_CMAC_256],
            });

        establishment
            .server_negotiation
            .get_or_insert(ServerNegotiationRecord {
                server_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            });

        establishment
            .port_negotiation
            .get_or_insert(PortNegotiationRecord { port: 4406 });

        self.build()
    }

    /// verifies that there are no ```ClientEstablishment``` fields set to ```None```.
    /// Then, returns pointer to the ```ClientEstablishment``` struct.
    pub fn build(&self) -> Result<&ClientEstablishment, rustntp::Error> {
        let establishment = &self.client_establishment;

        establishment
            .end_of_message
            .ok_or(rustntp::Error::MissingEstablishmentRecord(String::from(
                "end of message",
            )));

        establishment
            .next_protocol_negotiation
            .ok_or(rustntp::Error::MissingEstablishmentRecord(String::from(
                "next protocol negotiation",
            )));

        establishment
            .aead_algorithm_negotiation
            .ok_or(rustntp::Error::MissingEstablishmentRecord(String::from(
                "AEAD algorithm negotiation",
            )));

        establishment
            .server_negotiation
            .ok_or(rustntp::Error::MissingEstablishmentRecord(String::from(
                "server negotiation",
            )));

        establishment
            .port_negotiation
            .ok_or(rustntp::Error::MissingEstablishmentRecord(String::from(
                "port negotiation",
            )));

        Ok(&self.client_establishment)
    }
}
