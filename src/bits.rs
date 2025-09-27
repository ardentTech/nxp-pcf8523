// Converts a BCD value to binary format
pub(crate) const fn decode_bcd(a: u8) -> u8 {
    (((a >> 4) & 0xf) * 10) + (a & 0xf)
}

// Converts a binary value to BCD format
pub(crate) const fn encode_bcd(a: u8) -> u8 {
    assert!(a < 100);
    (a % 10) | (a / 10) << 4
}

// Get a subset of bits from a byte
pub(crate) const fn get_bits(byte: u8, num_bits: u8, lsb_offset: u8) -> u8 {
    (byte >> lsb_offset) & ((1 << (num_bits)) - 1)
}

// Set bits on a byte
pub(crate) const fn set_bits(byte: &mut u8, bits: u8, lsb_offset: u8, mask: u8) {
    *byte &= !mask;
    *byte |= (bits << lsb_offset) & mask
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
