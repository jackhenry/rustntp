// #![allow(dead_code)]
use tokio::net::TcpStream;
use tokio_rustls::TlsStream;

pub struct TcpRecord {
    payload: Vec<u8>,
}
pub trait Protocol {
    fn deliver(&self, stream: TlsStream<TcpStream>) -> ();
}

pub struct ClientEstablishment {
    end_of_message: EndOfMessageRecord,
    next_protocol_negotiation: NextProtocolNegotiationRecord,
    aead_algorithm_negotiation: ClientAEADAlgorithmRecord,
    server_negotiation: ServerNegotiationRecord,
    port_negotiation: PortNegotiationRecord,
}

impl Protocol for ClientEstablishment {
    fn deliver(&self, _stream: TlsStream<TcpStream>) -> () {
        todo!("Not implemented");
    }
}

pub struct ServerEstablishment {
    end_of_message: EndOfMessageRecord,
    next_protocol_negotiation: NextProtocolNegotiationRecord,
    error_notification: ErrorRecord,
    warning_notification: WarningRecord,
    aead_algorithm_negotiation: ServerAEADAlgorithmRecord,
    new_cookie: NewCookieRecord,
    server_negotiation: ServerNegotiationRecord,
    port_negotiation: PortNegotiationRecord,
}

impl Protocol for ServerEstablishment {
    fn deliver(&self, _record: Vec<TcpRecord>, _stream: TlsStream<TcpStream>) -> () {
        todo!("Not implemented");
    }
}

pub trait NTSKERecord {
    fn to_tcp_record(&self) -> TcpRecord;
}

pub struct EndOfMessageRecord {}

impl NTSKERecord for EndOfMessageRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

pub struct NextProtocolNegotiationRecord {
    protocol_ids: Vec<u16>,
}

impl NTSKERecord for NextProtocolNegotiationRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

pub struct ErrorRecord {
    error_code: u16,
}

impl NTSKERecord for ErrorRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

pub struct WarningRecord {
    warning_code: u16,
}

impl NTSKERecord for WarningRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

/// Server AEAD negotiation can only return one 16 bit uint
pub struct ServerAEADAlgorithmRecord {
    algorithm: u16,
}

impl NTSKERecord for ServerAEADAlgorithmRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

/// Client AEAD negotiation can return an array of 16 bit uint
pub struct ClientAEADAlgorithmRecord {
    algorithms: Vec<u16>,
}

impl NTSKERecord for ClientAEADAlgorithmRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

pub struct Cookie {
    index: u16,
    nonce: u16,
    cipher: u32,
}

pub struct NewCookieRecord {
    cookies: Vec<Cookie>,
}

impl NTSKERecord for NewCookieRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

pub struct ServerNegotiationRecord {
    server_address: String,
}

impl NTSKERecord for ServerNegotiationRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

pub struct PortNegotiationRecord {
    port: u16,
}

impl NTSKERecord for PortNegotiationRecord {
    fn to_tcp_record(&self) -> TcpRecord {
        todo!()
    }
}

pub enum NTSKERecordType {
    EndOfMessage,
    NextProtocolNegotiation,
    ProtocolError,
    Warning,
    AEADAlgorithmNegotiation,
    NewCookie,
    ServerNegotiation,
    PortNegotiation,
}
