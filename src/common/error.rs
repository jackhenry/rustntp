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
}
