use crate::protocol::NTS_NEXT_PROTOCOL_NTP_ID;
// IDs supported for next
pub const NTS_NEXT_PROTCOL_SUPPORTED_IDS: &'static [u16] = &[NTS_NEXT_PROTOCOL_NTP_ID];

// https://www.iana.org/assignments/aead-parameters/aead-parameters.xhtml
pub const AES_SIV_CMAC_256: u16 = 15;
pub const AES_SIV_CMAC_512: u16 = 17;

pub const SUPPORTED_AEAD_ALGORITHMS: &'static [u16] = &[AES_SIV_CMAC_256, AES_SIV_CMAC_512];
