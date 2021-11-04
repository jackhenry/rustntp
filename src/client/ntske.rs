use std::io::Write;
use std::net::TcpStream;

use crate::builder;
use crate::config::ClientConfig;
use crate::execution::ExecuteFnPtr;
use crate::execution::ExecutionCreator;
use crate::response::ExecutionResponseType::*;
use crate::response::NTSServers;
use crate::tls::TLS;
use rustntp::protocol;
use rustntp::support;
use tokio_rustls::TlsConnector;

#[derive(Debug)]
pub struct KeyEstablishment;

impl ExecutionCreator for KeyEstablishment {
    fn create_executable() -> ExecuteFnPtr {
        |config: &ClientConfig| {
            // The response object which holds the error and cookie maps for each server
            let mut response_data = NTSServers::new();
            let servers = config.servers.as_ref();

            let found_servers = servers.unwrap();

            for server in found_servers.iter() {
                let server_address = &server.address;
                let port = server.port.unwrap();
                let establishment = builder::ClientEstablishmentBuilder::new()
                    .next_protocol_negotiation(protocol::NTS_NEXT_PROTOCOL_NTP_ID)
                    .aead_algorithm_negotiation(vec![support::AES_SIV_CMAC_256])
                    .server_negotiation(server_address.to_string())
                    .port_negotiation(port)
                    .end_of_message()
                    .build();

                match establishment {
                    Err(error) => {
                        response_data
                            .error_map
                            .insert(server_address.to_string(), error);
                    }
                    Ok(built_establishment) => {
                        let buffer = built_establishment.network_buffer();
                        let mut conn = TLS::client_connection_for(&server_address);
                        let mut socket =
                            TcpStream::connect(format!("{}:{}", &server_address, port)).unwrap();
                        let mut tls = rustls::Stream::new(&mut conn, &mut socket);
                        println!("{:?}", buffer);
                        tls.write(&buffer.as_slice());
                    }
                };
            }

            KeyEstablishmentResponse(response_data)
        }
    }
}
