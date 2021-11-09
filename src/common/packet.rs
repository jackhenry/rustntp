use crate::helper;
use crate::protocol::ntp::Timestamp;

#[derive(Debug)]
pub struct NTPPacket {
    leap: u8,
    version: u8,
    mode: u8,
    stratum: u8,
    poll: u8,
    precision: i8,
    root_delay: f32,
    root_dispersion: f32,
    ref_id: Option<String>,
    reference: Timestamp,
    originate: Timestamp,
    receive: Timestamp,
    transmit: Timestamp,
}

impl NTPPacket {
    pub fn from(buffer: &[u8]) -> Self {
        let leap = buffer[0] >> 6;
        let version = (buffer[0] & 0x38) >> 3;
        let mode = buffer[0] & 0x7;
        let stratum = buffer[1];
        let poll = buffer[2];
        let precision = buffer[3] as i8;
        let root_delay = helper::to_ntp_floating(&buffer[4..8]);
        let root_dispersion = helper::to_ntp_floating(&buffer[8..12]);
        let ref_id = String::from_utf8(Vec::from(&buffer[12..16])).ok();
        let reference = Timestamp::from(&buffer[16..24], precision);
        let originate = Timestamp::from(&buffer[24..32], precision);
        let receive = Timestamp::from(&buffer[32..40], precision);
        let transmit = Timestamp::from(&buffer[40..48], precision);

        Self {
            leap,
            version,
            mode,
            stratum,
            poll,
            precision,
            root_delay,
            root_dispersion,
            ref_id,
            reference,
            originate,
            receive,
            transmit,
        }
    }
}

#[cfg(test)]
mod test {
    use super::NTPPacket;

    #[test]
    fn packet_creation() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut test_buffer: [u8; 48] = [0; 48];

        for i in 0..48 {
            test_buffer[i] = rng.gen();
        }
        // precision
        test_buffer[3] = 250;
        //e5 34 46 db 9a eb c7 8a
        test_buffer[40] = 229;
        test_buffer[41] = 52;
        test_buffer[42] = 70;
        test_buffer[43] = 219;
        test_buffer[44] = 154;
        test_buffer[45] = 235;
        test_buffer[46] = 199;
        test_buffer[47] = 138;

        let packet = NTPPacket::from(&test_buffer);
        println!("{:?}", packet);
        assert_eq!(packet.transmit.seconds, 3845408475);
        assert_eq!(packet.transmit.fraction, 0.605161);
    }
}
