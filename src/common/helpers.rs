pub mod helper {
    pub fn u16_to_i16(value: &u16) -> i16 {
        let sign = value >> 15;
        let converted = (value & 0x7FFF) as i16;
        let inverse = converted ^ 0x7FFF;

        return if sign == 1 {
            -1 * (inverse + 1)
        } else {
            converted
        };
    }

    pub fn combine_bytes_be(b0: &u8, b1: &u8) -> u16 {
        return ((*b0 as u16) << 8) | *b1 as u16;
    }

    pub fn combine_bytes_to_u32_be(b0: &u8, b1: &u8, b2: &u8, b3: &u8) -> u32 {
        let w0 = combine_bytes_be(b0, b1);
        let w1 = combine_bytes_be(b2, b3);
        return ((w0 as u32) << 16) | w1 as u32;
    }

    pub fn combine_u32_to_u64(u0: &u32, u1: &u32) -> u64 {
        return ((*u0 as u64) << 32) | *u1 as u64;
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
        let value = (*&leading as i16) as f32 + frac;
        value
    }
}
