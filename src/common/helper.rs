pub mod Helper {
    pub fn u16_to_i16(value: &u16) -> i16 {
        let sign = value >> 15;
        let converted = (value & 0x8FFF) as i16;

        return if sign == 1 { -1 * converted } else { converted };
    }

    pub fn combine_bytes_be(b0: &u8, b1: &u8) -> u16 {
        return ((*b0 as u16) << 8) | *b1 as u16;
    }

    pub fn to_ntp_floating(buffer_slice: &[u8]) -> f32 {
        let b0 = buffer_slice[0];
        let b1 = buffer_slice[1];
        let b2 = buffer_slice[2];
        let b3 = buffer_slice[3];
        let leading = combine_bytes_be(&b0, &b1);
        let right = combine_bytes_be(&b2, &b3);

        let leading_zeros = right.leading_zeros() as f32;
        let frac = right as f32 * 10f32.powf(-1.0 * (8.0 - leading_zeros));
        let value = u16_to_i16(&leading) as f32 + frac;
        value
    }
}
