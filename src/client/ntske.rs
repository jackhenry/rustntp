use rustntp::establishment::ClientEstablishment;
use rustntp::protocol;
use rustntp::support;

use crate::builder;
use crate::config;
use crate::config::ClientConfig;
use crate::execution;
use crate::execution::ExecuteFnPtr;
use crate::execution::ExecutionCreator;
use crate::execution::ExecutionRequest;
use crate::response;
use crate::response::ExecutionResponseType;
use crate::response::KeyEstablismentResponse;

#[derive(Debug)]
pub struct KeyEstablishment;

impl ExecutionCreator for KeyEstablishment {
    fn create_executable() -> ExecuteFnPtr {
        let closure = | config: &ClientConfig | {
            let establishers: Vec<Result<ClientEstablishment, rustntp::Error>> = 
                config
                .servers
                .as_ref()
                .unwrap()
                .iter()
                .map(|server| {
                    let server_address = &server.address;
                    let port = server.port.unwrap();
                    builder::ClientEstablishmentBuilder::new()
                        .next_protocol_negotiation(protocol::NTS_NEXT_PROTOCOL_NTP_ID)
                        .aead_algorithm_negotiation(vec![support::AES_SIV_CMAC_256])
                        .server_negotiation(server_address.to_string())
                        .port_negotiation(port)
                        .end_of_message()
                        .build()
                })
                .collect();

            ExecutionResponseType::KeyEstablishment(KeyEstablismentResponse {})
        };
        return closure;
    }
}
