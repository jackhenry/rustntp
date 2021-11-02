use std::{net::IpAddr, panic};

use argh::FromArgs;
use rustntp::{tcp::{PacketData, TcpStreamable}};

use crate::builder::ClientEstablishmentBuilder;

mod connection;
mod builder;

#[derive(FromArgs)]
/// Client arguments
struct ClientArguments {
    #[argh(positional)]
    host: String,

    #[argh(positional)]
    port: u16,
}

fn main() -> std::io::Result<()> {
    
    let options: ClientArguments = argh::from_env();
    
    let ipaddr = match options.host.parse::<IpAddr>() {
        Ok(ipaddr) => ipaddr,
        Err(_) => panic!("Invalid ip address provided."),
    };
    
    let port = options.port;
    
    let url = format!("{}:{}", ipaddr, port);
    println!("url: {}", url);
    
    let establishment = ClientEstablishmentBuilder::new()
        .next_protocol_negotiation(rustntp::protocol::NTS_NEXT_PROTOCOL_NTP_ID)
        .aead_algorithm_negotiation(vec![rustntp::support::AES_SIV_CMAC_256])
        .server_negotiation(ipaddr)
        .port_negotiation(port)
        .end_of_message()
        .build().unwrap();
    let mut tcp_record = establishment.port_negotiation.to_tcp_record();
    println!("{:?}", tcp_record);
    println!("{:?}", tcp_record.to_network_buffer().raw_packet_str());

    Ok(())
}
