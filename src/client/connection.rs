use webpki::{DnsNameRef, InvalidDnsNameError};

pub struct NTSConnection {
    con_str: String,
}

impl NTSConnection {
    pub fn new(host: String, port: u16) -> Result<NTSConnection, InvalidDnsNameError> {
        DnsNameRef::try_from_ascii_str(&host)?;
        let con_str = format!("{}:{}", host, port);
        Ok(NTSConnection { con_str })
    }
    fn key_establishment(&self) {}
}
