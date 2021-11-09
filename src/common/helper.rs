pub mod Helper {
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

    pub fn to_floating_point(leading: u32, fraction_part: u32) -> f32 {
        let leading_zeros = fraction_part.leading_zeros() as f32;
        let fraction = leading as f32 * 10f32.powf(-1.0 * (32.0 - leading_zeros));
        return (leading as f32) + fraction;
    }

    pub fn fraction_bits_to_decimal(precision: i8, frac: u32) -> f32 {
        return (10f32.powf(-1.0 * precision as f32)) * (frac as f32 / 2f32.powf(32.0));
    }
}
