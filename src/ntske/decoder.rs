use rustntp::establishment::NTSPacket;

pub struct Decoder {}

impl Decoder {
    pub fn decode_stream(buffer: &[u8]) -> Result<Vec<NTSPacket>, rustntp::Error> {
        let mut buffer = buffer.clone();
        let mut records: Vec<NTSPacket> = Vec::new();

        // A record requires 4 bytes at minimum
        // Therefore, if less than 4, there can be no more records
        while buffer.len() >= 4 {
            let decode_result = NTSPacket::decode(buffer);
            if let Ok(record) = decode_result {
                let end = record.body_length as usize;
                buffer = &buffer[0..end - 4];
                records.push(record);
                println!("{}", buffer.len());
            } else {
                return Err(decode_result.unwrap_err());
            }
        }

        Ok(records)
    }

    fn next_packet(mut buffer: &[u8]) -> Result<NTSPacket, rustntp::Error> {
        let buffer_len = buffer.len();
        // Buffer must have at least 4 bytes (critical bit, record type, body length)
        if buffer_len < 4 {
            return Err(rustntp::Error::NTSEstablishmentDecodeError(String::from(
                "Buffer too small to hold minimum amount of data.",
            )));
        }

        let first_byte = buffer[0];
        let second_byte = buffer[1];
        let critical_bit = (first_byte >> 7) == 1;
        let record_type = (((first_byte & 0x7f) << 8) + second_byte) as u16;
        let body_length = ((buffer[2] as u16) << 8) | buffer[3] as u16;
        // end of current record in buffer
        let end = 4 + (body_length as usize) - 1;

        let mut body: Vec<u8> = Vec::new();
        if body_length > 0 {
            body = Vec::from(&buffer[4..end]);
        }
        buffer = &buffer[end..];

        Ok(NTSPacket {
            critical_bit,
            record_type,
            body_length,
            body,
        })
    }
}
