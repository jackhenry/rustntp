use std::net::IpAddr;

use rustntp::establishment::ClientEstablishment;
use rustntp::protocol::ntske::*;
use rustntp::support;

#[derive(Debug)]
pub struct ClientEstablishmentBuilder {
    pub end_of_message: Option<EndOfMessageRecord>,
    pub next_protocol_negotiation: Option<NextProtocolNegotiationRecord>,
    pub aead_algorithm_negotiation: Option<ClientAEADAlgorithmRecord>,
    pub server_negotiation: Option<ServerNegotiationRecord>,
    pub port_negotiation: Option<PortNegotiationRecord>,
}

impl ClientEstablishmentBuilder {
    pub fn new() -> ClientEstablishmentBuilder {
        ClientEstablishmentBuilder {
            end_of_message: None,
            next_protocol_negotiation: None,
            aead_algorithm_negotiation: None,
            server_negotiation: None,
            port_negotiation: None,
        }
    }

    pub fn end_of_message(mut self) -> Self {
        self.end_of_message = Some(EndOfMessageRecord {});
        self
    }

    pub fn next_protocol_negotiation(mut self, protocol: u16) -> Self {
        let protocol_ids = vec![protocol];
        self.next_protocol_negotiation = Some(NextProtocolNegotiationRecord { protocol_ids });
        self
    }

    pub fn aead_algorithm_negotiation(mut self, algorithms: Vec<u16>) -> Self {
        let invalid_algorithm = |algorithm| !support::SUPPORTED_AEAD_ALGORITHMS.contains(algorithm);

        if algorithms.iter().any(invalid_algorithm) {}

        self.aead_algorithm_negotiation = Some(ClientAEADAlgorithmRecord { algorithms });
        self
    }

    pub fn server_negotiation(mut self, server_address: String) -> Self {
        self.server_negotiation = Some(ServerNegotiationRecord { server_address });
        self
    }

    pub fn port_negotiation(mut self, port: u16) -> Self {
        self.port_negotiation = Some(PortNegotiationRecord { port });
        self
    }

    fn validate(&self) -> Result<&Self, rustntp::Error> {
        // Verify that no fields are set to None
        self.end_of_message
            .as_ref()
            .ok_or(rustntp::Error::MissingEstablishmentRecord(String::from(
                "end of message",
            )))?;
        self.next_protocol_negotiation.as_ref().ok_or(
            rustntp::Error::MissingEstablishmentRecord(String::from("next protocol negotiation")),
        )?;
        self.aead_algorithm_negotiation.as_ref().ok_or(
            rustntp::Error::MissingEstablishmentRecord(String::from("aead algorithm negotiation")),
        )?;
        self.server_negotiation
            .as_ref()
            .ok_or(rustntp::Error::MissingEstablishmentRecord(String::from(
                "server negotiation",
            )))?;
        self.port_negotiation
            .as_ref()
            .ok_or(rustntp::Error::MissingEstablishmentRecord(String::from(
                "port negotiation",
            )))?;

        let protocol_ids = self
            .next_protocol_negotiation
            .as_ref()
            .unwrap()
            .protocol_ids
            .clone();
        let invalid_protcol_ids = |id: &u16| !support::NTS_NEXT_PROTCOL_SUPPORTED_IDS.contains(id);
        if let Some(id) = protocol_ids.into_iter().filter(invalid_protcol_ids).next() {
            return Err(rustntp::Error::UnsupportedNTSNextProtocolID(id));
        }

        let algorithm_ids = self
            .aead_algorithm_negotiation
            .as_ref()
            .unwrap()
            .algorithms
            .clone();
        let invalid_algorithm_ids = |id: &u16| !support::SUPPORTED_AEAD_ALGORITHMS.contains(id);
        if let Some(id) = algorithm_ids
            .into_iter()
            .filter(invalid_algorithm_ids)
            .next()
        {
            return Err(rustntp::Error::UnsupportedNTSAlgorithmID(id));
        }

        Ok(self)
    }

    pub fn build(self) -> Result<ClientEstablishment, rustntp::Error> {
        self.validate()?;

        Ok(ClientEstablishment {
            end_of_message: self.end_of_message.unwrap(),
            next_protocol_negotiation: self.next_protocol_negotiation.unwrap(),
            aead_algorithm_negotiation: self.aead_algorithm_negotiation.unwrap(),
            server_negotiation: self.server_negotiation.unwrap(),
            port_negotiation: self.port_negotiation.unwrap(),
        })
    }
}
