use std::fmt::{Binary, Debug};
use std::mem::size_of;

use crate::protocol::ntske::{ClientAEADAlgorithmRecord, EndOfMessageRecord, ErrorRecord, NextProtocolNegotiationRecord, PortNegotiationRecord, ServerNegotiationRecord};
use crate::protocol;



#[derive(Debug)]
pub struct TcpRecord {
    pub critical_bit: bool,
    pub record_type: u16,
    pub body_length: u16,
    pub body: Vec<u8>,
}

impl TcpRecord {
    pub fn to_network_buffer(&mut self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        
        let crit_and_record_type_bytes = match self.critical_bit {
            true => self.record_type | (0x1u16 << 15),
            false => self.record_type & (0xFFFFu16 >> 1)
        };
        let body_length_bytes = self.body_length;
        
        // 1 bit critical indicator and 15 bit record type
        buffer.append(&mut crit_and_record_type_bytes.to_be_bytes().to_vec());
        // 16 bit body length
        buffer.append(&mut body_length_bytes.to_be_bytes().to_vec());
        // variable length body
        buffer.append(&mut self.body.to_vec());
        buffer
    }
}

pub trait TcpStreamable {
    fn to_tcp_record(&self) -> TcpRecord;
}

impl TcpStreamable for EndOfMessageRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        return TcpRecord {
            critical_bit: true, //Per RFC, critical bit must be set to 1
            record_type: protocol::ntske::END_OF_MESSAGE_RECORD_TYPE_NUMBER,
            body_length: 0,
            body: Vec::new()
        }
    }
}

impl TcpStreamable for NextProtocolNegotiationRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        let protocol_ids = &self.protocol_ids;
        let mut body: Vec<u8> = vec![0x0; protocol_ids.len() * 2];
        let mut body_length: u16 = 0;
        // 0x1234 becomes [0x12, 0x34] and are inserted into body
        for (i, &id) in protocol_ids.iter().enumerate() {
            let bytes = id.to_be_bytes();
            body[i * 2] = bytes[0];
            body[i * 2 + 1] = bytes[1];
            body_length += 2;
        }
        
        TcpRecord {
            critical_bit: true,
            record_type: protocol::ntske::NTS_NEXT_PROTOCOL_RECORD_TYPE_NUMBER,
            body_length,
            body
        }
    }
}

impl TcpStreamable for ClientAEADAlgorithmRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        let algorithm_ids = &self.algorithms;
        let mut body: Vec<u8> = vec![0x0; algorithm_ids.len() * 2];
        let mut body_length: u16 = 0;
        for (i , &id) in algorithm_ids.iter().enumerate() {
            let bytes = id.to_be_bytes();
            body[i * 2] = bytes[0];
            body[i * 2 + 1] = bytes[1];
            body_length += 2;
        } 
        
        TcpRecord {
            // RFC states that critical bit MAY be set
            critical_bit: true,
            record_type: protocol::ntske::AEAD_ALGORITHM_RECORD_TYPE_NUMBER,
            body_length,
            body
        }
    }
}

impl TcpStreamable for ServerNegotiationRecord {
    fn to_tcp_record(&self) -> TcpRecord {

        let body: Vec<u8> = self.server_address.to_string().into_bytes();
        let mut body_length: u16 = 0;
        for _ in body.iter() {
            body_length += 1;
        }
        
        TcpRecord {
            // Per RFC, clients do not set critical bit
            critical_bit: false,
            record_type: protocol::ntske::NTPV4_SERVER_NEGOTIATION_RECORD_TYPE,
            body_length,
            body
        }
    }
}

impl TcpStreamable for PortNegotiationRecord {
    fn to_tcp_record(&self) -> TcpRecord {

        let port = self.port;
        // Divide the 16 bit port into two bytes (big endian) and push to body
        let port_bytes = port.to_be_bytes();
        let mut body: Vec<u8> = vec![0x0; 2];
        body[0] = port_bytes[0];
        body[1] = port_bytes[1];
        let mut body_length: u16 = 0;
        for _ in body.iter() {
            body_length += 1;
        }
        
        TcpRecord {
            // Per RFC, clients do not set critical bit
            critical_bit: false,
            record_type: protocol::ntske::NTPV4_PORT_NEGOTIATION_RECORD_TYPE,
            body_length,
            body
        }
    }
}

pub trait PacketData<T> {
    fn raw_packet_str(&self) -> ();
}

impl<T> PacketData<T> for Vec<T>
where
    T: Binary + Debug,
{
    fn raw_packet_str(&self) {
        //self.iter().for_each(|v| println!("{:?}", v));
        println!("{:?}", self.iter());
        for (i, v) in self.iter().enumerate() {
            println!("${:?}", v);
            let binary = format!("{:b}", v);
            if i == 0 {
                //print!("{:0>8} ", binary);
            } else if i % 4 != 0 {
                //print!("{:0>8} ", binary);
            } else {
                //print!("\n{:0>8} ", binary);
            }
        }
    }
}