
pub(crate) fn decode_bcd(a: u8) -> u8 {
    (((a >> 4) & 0xf) * 10) + (a & 0xf)
}

pub(crate) fn encode_bcd(a: u8) -> u8 {
    if a >= 100 { panic!("Cannot BCD encode value {} as u8", a); }
    a % 10 | (a / 10 << 4)
}

pub(crate) fn get_bits(byte: u8, bits: u8, lsb_offset: u8) -> u8 {
    (byte >> lsb_offset) & ((1 << (bits)) - 1)
}

// TOOD i think there's a way to dyn gen the mask based upon data and lsb_offset
pub(crate) fn set_bits(byte: &mut u8, data: u8, lsb_offset: u8, mask: u8) {
    *byte &= !mask;
    *byte |= (data << lsb_offset) & mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_bcd_floor() {
        assert_eq!(decode_bcd(0b0), 0u8);
    }

    #[test]
    fn decode_bcd_ceiling() {
        assert_eq!(decode_bcd(0b1001_1001), 99u8);
    }

    #[test]
    fn encode_bcd_floor() {
        assert_eq!(encode_bcd(0u8), 0b0);
    }

    #[test]
    fn encode_bcd_ceiling() {
        assert_eq!(encode_bcd(99u8), 0b1001_1001);
    }

    #[test]
    #[should_panic]
    fn encode_bcd_out_of_bounds() {
        encode_bcd(100u8);
    }
}