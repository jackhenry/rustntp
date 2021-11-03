use rustntp::establishment::ClientEstablishment;
use rustntp::protocol;
use rustntp::support;

use crate::builder;
use crate::config;
use crate::execution;
use crate::execution::ExecutionRequest;
use crate::response;
use crate::response::ExecutionResponseType;
use crate::response::KeyEstablismentResponse;

#[derive(Debug)]
pub struct KeyEstablishment {
    server: config::NTSKeyExchangeServer,
}

impl KeyEstablishment {
    pub fn new(server: config::NTSKeyExchangeServer) -> Self {
        Self { server }
    }
}

impl ExecutionRequest for KeyEstablishment {
    type ResponseType = ExecutionResponseType;
    fn execute(&self) -> Self::ResponseType {
        let port = self
            .server
            .port
            .unwrap_or(4406);

        let server_address = String::from("127.0.0.1");

        let establishment = builder::ClientEstablishmentBuilder::new()
            .next_protocol_negotiation(protocol::NTS_NEXT_PROTOCOL_NTP_ID)
            .aead_algorithm_negotiation(vec![support::AES_SIV_CMAC_256])
            .server_negotiation(server_address)
            .port_negotiation(port)
            .end_of_message()
            .build();

        ExecutionResponseType::KeyEstablishment(KeyEstablismentResponse {})
    }
}
