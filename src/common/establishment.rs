use crate::protocol;
use crate::protocol::ntske::*;

#[derive(Debug)]
pub struct ClientEstablishment {
    pub end_of_message: EndOfMessageRecord,
    pub next_protocol_negotiation: NextProtocolNegotiationRecord,
    pub aead_algorithm_negotiation: ClientAEADAlgorithmRecord,
    pub server_negotiation: ServerNegotiationRecord,
    pub port_negotiation: PortNegotiationRecord,
}

impl ClientEstablishment {
    pub fn network_buffer(&self) -> Vec<u8> {
        let mut next_proto_buffer = self.next_protocol_negotiation.nts_packet().buffer();
        let mut aead_algo_buffer = self.aead_algorithm_negotiation.nts_packet().buffer();
        let mut server_negotiation_buffer = self.server_negotiation.nts_packet().buffer();
        let mut port_negotiation_buffer = self.port_negotiation.nts_packet().buffer();
        let mut end_of_message_buffer = self.end_of_message.nts_packet().buffer();

        let mut buffer = Vec::new();
        buffer.append(&mut next_proto_buffer);
        buffer.append(&mut aead_algo_buffer);
        buffer.append(&mut server_negotiation_buffer);
        buffer.append(&mut port_negotiation_buffer);
        buffer.append(&mut end_of_message_buffer);

        return buffer;
    }
}

pub struct ServerEstablishment {
    pub end_of_message: EndOfMessageRecord,
    pub next_protocol_negotiation: NextProtocolNegotiationRecord,
    pub error_notification: ErrorRecord,
    pub warning_notification: WarningRecord,
    pub aead_algorithm_negotiation: ServerAEADAlgorithmRecord,
    pub new_cookie: NewCookieRecord,
    pub server_negotiation: ServerNegotiationRecord,
    pub port_negotiation: PortNegotiationRecord,
}

impl ServerEstablishment {}

#[derive(Debug)]
pub struct NTSPacket {
    pub critical_bit: bool,
    pub record_type: u16,
    pub body_length: u16,
    pub body: Vec<u8>,
}

impl NTSPacket {
    /// Returns a `vec[u8]` byte buffer that adheres to RFC 8195 requirements for NTS Records
    pub fn buffer(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let crit_and_record_type_bytes = match self.critical_bit {
            true => self.record_type | (0x1u16 << 15),
            false => self.record_type & (0xFFFFu16 >> 1),
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

pub trait NTSPacketTransform {
    fn nts_packet(&self) -> NTSPacket;
}

impl NTSPacketTransform for EndOfMessageRecord {
    fn nts_packet(&self) -> NTSPacket {
        return NTSPacket {
            critical_bit: true, //Per RFC, critical bit must be set to 1
            record_type: protocol::ntske::END_OF_MESSAGE_RECORD_TYPE_NUMBER,
            body_length: 0,
            body: Vec::new(),
        };
    }
}

impl NTSPacketTransform for NextProtocolNegotiationRecord {
    fn nts_packet(&self) -> NTSPacket {
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

        NTSPacket {
            critical_bit: true,
            record_type: protocol::ntske::NTS_NEXT_PROTOCOL_RECORD_TYPE_NUMBER,
            body_length,
            body,
        }
    }
}

impl NTSPacketTransform for ClientAEADAlgorithmRecord {
    fn nts_packet(&self) -> NTSPacket {
        let algorithm_ids = &self.algorithms;
        let mut body: Vec<u8> = vec![0x0; algorithm_ids.len() * 2];
        let mut body_length: u16 = 0;
        for (i, &id) in algorithm_ids.iter().enumerate() {
            let bytes = id.to_be_bytes();
            body[i * 2] = bytes[0];
            body[i * 2 + 1] = bytes[1];
            body_length += 2;
        }

        NTSPacket {
            // RFC states that critical bit MAY be set
            critical_bit: true,
            record_type: protocol::ntske::AEAD_ALGORITHM_RECORD_TYPE_NUMBER,
            body_length,
            body,
        }
    }
}

impl NTSPacketTransform for ServerNegotiationRecord {
    fn nts_packet(&self) -> NTSPacket {
        let body: Vec<u8> = self.server_address.to_string().into_bytes();
        let mut body_length: u16 = 0;
        for _ in body.iter() {
            body_length += 1;
        }

        NTSPacket {
            // Per RFC, clients do not set critical bit
            critical_bit: false,
            record_type: protocol::ntske::NTPV4_SERVER_NEGOTIATION_RECORD_TYPE,
            body_length,
            body,
        }
    }
}

impl NTSPacketTransform for PortNegotiationRecord {
    fn nts_packet(&self) -> NTSPacket {
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

        NTSPacket {
            // Per RFC, clients do not set critical bit
            critical_bit: false,
            record_type: protocol::ntske::NTPV4_PORT_NEGOTIATION_RECORD_TYPE,
            body_length,
            body,
        }
    }
}
