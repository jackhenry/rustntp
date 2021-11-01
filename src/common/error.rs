#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// NTP is the only currently supported protocol for NTS
    #[error("NTP is currently the only supported protocol")]
    UnsupportedProtocol,

    #[error("Unsupported AEAD algorithm")]
    UnsupportedAlgorithm,

    #[error("Client establishment is missing `{0}` record")]
    MissingEstablishmentRecord(String),
}
