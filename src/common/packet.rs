use crate::protocol::ntp::Timestamp;
use crate::Helper;

#[derive(Debug)]
pub struct NTPPacket {
    leap: u8,
    version: u8,
    mode: u8,
    stratum: u8,
    poll: u8,
    precision: u8,
    root_delay: f32,
    root_dispersion: f32,
    ref_id: String,
    reference: Timestamp,
    originate: Timestamp,
    receive: Timestamp,
    transmit: Timestamp,
}

impl NTPPacket {
    pub fn from(buffer: &[u8]) -> Self {
        //00011
        let leap = buffer[0] >> 6;
        let version = (buffer[0] & 0x38) >> 3;
        let mode = buffer[0] & 0x7;
        let stratum = buffer[1];
        let poll = buffer[2];
        let precision = buffer[3];
        println!("Getting root delay");
        let root_delay = Helper::to_ntp_floating(&buffer[4..8]);
        println!("Getting root dispersion");
        let root_dispersion = Helper::to_ntp_floating(&buffer[8..12]);

        Self {
            leap,
            version,
            mode,
            stratum,
            poll,
            precision,
            root_delay,
            root_dispersion,
            ref_id: String::from("test"),
            reference: Timestamp { value: 0.0 },
            originate: Timestamp { value: 0.0 },
            receive: Timestamp { value: 0.0 },
            transmit: Timestamp { value: 0.0 },
        }
    }
}
