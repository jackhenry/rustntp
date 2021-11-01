pub const NTP_PROTOCOL_ID: u16 = 1;

pub mod ntske {
    use std::net::IpAddr;

    #[derive(Debug)]
    pub struct TcpRecord {
        pub payload: Vec<u8>,
    }

    #[derive(Debug)]
    pub struct EndOfMessageRecord {}

    #[derive(Debug)]
    pub struct NextProtocolNegotiationRecord {
        pub protocol_ids: Vec<u16>,
    }

    #[derive(Debug)]
    pub struct ErrorRecord {
        pub error_code: u16,
    }

    #[derive(Debug)]
    pub struct WarningRecord {
        pub warning_code: u16,
    }

    /// Server AEAD negotiation can only return one 16 bit uint
    #[derive(Debug)]
    pub struct ServerAEADAlgorithmRecord {
        pub algorithm: u16,
    }

    /// Client AEAD negotiation can return an array of 16 bit uint
    #[derive(Debug)]
    pub struct ClientAEADAlgorithmRecord {
        pub algorithms: Vec<u16>,
    }

    #[derive(Debug)]
    pub struct Cookie {
        pub index: u16,
        pub nonce: u16,
        pub cipher: u32,
    }

    #[derive(Debug)]
    pub struct NewCookieRecord {
        pub cookies: Vec<Cookie>,
    }

    #[derive(Debug)]
    pub struct ServerNegotiationRecord {
        pub server_address: IpAddr,
    }

    #[derive(Debug)]
    pub struct PortNegotiationRecord {
        pub port: u16,
    }
}
