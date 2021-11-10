use fixed::traits::Fixed;
use fixed::types::extra::U16;
use fixed::types::extra::U32;
use fixed::FixedU32;
use fixed::FixedU64;
use typed_builder::TypedBuilder;

use crate::systime::SystemTime;

#[derive(Debug, TypedBuilder)]
pub struct NTPPacket {
    pub leap: u8,
    pub version: u8,
    pub mode: u8,
    pub stratum: u8,
    pub poll: u8,
    pub precision: i8,
    pub root_delay: FixedU32<U16>,
    pub root_dispersion: FixedU32<U16>,
    pub ref_id: [u8; 4],
    pub reference: FixedU64<U32>,
    pub originate: FixedU64<U32>,
    pub receive: FixedU64<U32>,
    pub transmit: FixedU64<U32>,
}

impl NTPPacket {
    pub fn from(buffer: &[u8]) -> Self {
        let leap = buffer[0] >> 6;
        let version = (buffer[0] & 0x38) >> 3;
        let mode = buffer[0] & 0x7;
        let stratum = buffer[1];
        let poll = buffer[2];
        let precision = buffer[3] as i8;
        let root_delay =
            FixedU32::<U16>::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let root_dispersion =
            FixedU32::<U16>::from_be_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);
        let ref_id: [u8; 4] = [buffer[12], buffer[13], buffer[14], buffer[15]];
        // buffer[16..24]
        let reference = FixedU64::<U32>::from_be_bytes([
            buffer[16], buffer[17], buffer[18], buffer[19], buffer[20], buffer[21], buffer[22],
            buffer[23],
        ]);

        // buffer[24..32]
        let originate = FixedU64::<U32>::from_be_bytes([
            buffer[24], buffer[25], buffer[26], buffer[27], buffer[28], buffer[29], buffer[30],
            buffer[31],
        ]);

        // buffer[32..40]
        let receive = FixedU64::<U32>::from_be_bytes([
            buffer[32], buffer[33], buffer[34], buffer[35], buffer[36], buffer[37], buffer[38],
            buffer[39],
        ]);

        // buffer[40..48]
        let transmit = FixedU64::<U32>::from_be_bytes([
            buffer[40], buffer[41], buffer[42], buffer[43], buffer[44], buffer[45], buffer[46],
            buffer[47],
        ]);

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

    pub fn mark_received(&mut self) {
        let default = FixedU64::<U32>::from_num(0.0);
        self.receive = SystemTime::get_ntp_epoch().unwrap_or(default);
    }
    pub fn mark_for_transmission(&mut self) {
        self.originate = self.transmit.clone();
        let default = FixedU64::<U32>::from_num(0.0);
        self.transmit = SystemTime::get_ntp_epoch().unwrap_or(default);
    }

    pub fn to_network_bytes(&self) -> Vec<u8> {
        let mut byte0 = 0;
        byte0 = (byte0 | self.leap) << 6;
        byte0 = (byte0 | self.version) << 3;
        byte0 = byte0 | 4u8;
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(byte0);
        buffer.push(1);
        buffer.push(17);
        buffer.push(self.precision as u8);
        buffer.append(&mut Vec::from(self.root_delay.to_be_bytes()));
        buffer.append(&mut Vec::from(self.root_dispersion.to_be_bytes()));

        buffer.append(&mut Vec::from(self.ref_id));

        println!("{:?}", self.transmit);
        println!("{:?}", self.transmit.to_be_bytes());
        buffer.append(&mut Vec::from(self.reference.to_be_bytes()));
        buffer.append(&mut Vec::from(self.originate.to_be_bytes()));
        buffer.append(&mut Vec::from(self.receive.to_be_bytes()));
        buffer.append(&mut Vec::from(self.transmit.to_be_bytes()));
        buffer
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
        // root delay
        test_buffer[4] = 0;
        test_buffer[5] = 1;
        test_buffer[6] = 250;
        test_buffer[7] = 0;
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
    }
}
