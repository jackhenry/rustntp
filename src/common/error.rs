#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// NTP is the only currently supported protocol for NTS Next Protocol Negotiation
    #[error("Protocol with id `{0}` is not supported")]
    UnsupportedNTSNextProtocolID(u16),

    #[error("AEAD Algorithm with id `{0}` does not exist or is not supported")]
    UnsupportedNTSAlgorithmID(u16),

    #[error("Client establishment is missing `{0}` record")]
    MissingEstablishmentRecord(String),

    #[error("Client config could not be read")]
    InvalidClientConfig,

    #[error("No valid NTS-KE server in config")]
    MissingNTSKEServerInConfig,

    #[error("Unable to decode client establishment records")]
    ClientEstablishmentDecodeError,

    #[error("{0}")]
    NTSEstablishmentDecodeError(String),

    #[error("{0}")]
    InvalidNTPPacket(String),

    #[error("{0}")]
    ClientNotReachable(String),

    #[error("{0}")]
    SystemTimeError(String),
}
