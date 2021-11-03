// IANA NTS Next Protocol Registry
// Only NTP is supported current
pub const NTS_NEXT_PROTOCOL_NTP_ID: u16 = 0;

pub mod ntske {
    pub const DEFAULT_NTS_PORT: u16 = 4406;
    // Recrod type numbers defined in RFC 8915
    pub const END_OF_MESSAGE_RECORD_TYPE_NUMBER: u16 = 0;
    pub const NTS_NEXT_PROTOCOL_RECORD_TYPE_NUMBER: u16 = 1;
    pub const ERROR_MESSAGE_RECORD_TYPE_NUMBER: u16 = 2;
    pub const WARNING_MESSAGE_RECORD_TYPE_NUMBER: u16 = 3;
    pub const AEAD_ALGORITHM_RECORD_TYPE_NUMBER: u16 = 4;
    pub const NEW_COOKIE_NTPV4_RECORD_TYPE_NUMBER: u16 = 5;
    pub const NTPV4_SERVER_NEGOTIATION_RECORD_TYPE: u16 = 6;
    pub const NTPV4_PORT_NEGOTIATION_RECORD_TYPE: u16 = 7;

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
        pub server_address: String,
    }

    #[derive(Debug)]
    pub struct PortNegotiationRecord {
        pub port: u16,
    }
}
