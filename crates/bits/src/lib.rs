pub fn decimal_to_binary(x: u8) -> String {
    format!("{:08b}", x)
}

pub fn binary_to_decimal(bits: &str) -> u8 {
    u8::from_str_radix(bits, 2).expect("invalid binary string")
}

pub fn add_u8(a: u8, b: u8) -> u8 {
    let mut sum = 0u8;
    let mut carry = 0u8;
    for bit in 0..8 {
        let abit = (a >> bit) & 1;
        let bbit = (b >> bit) & 1;
        let s = abit ^ bbit ^ carry;
        carry = (abit & bbit) | (abit & carry) | (bbit & carry);
        sum |= s << bit;
    }
    sum
}

pub fn twos_complement(x: u8) -> u8 {
    (!x).wrapping_add(1)
}

pub fn sub_u8(a: u8, b: u8) -> u8 {
    add_u8(a, twos_complement(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_roundtrip() {
        for x in 0u8..=255 {
            assert_eq!(binary_to_decimal(&decimal_to_binary(x)), x);
        }
    }

    #[test]
    fn decimal_to_binary_pads_to_8() {
        assert_eq!(decimal_to_binary(13), "00001101");
        assert_eq!(decimal_to_binary(0), "00000000");
        assert_eq!(decimal_to_binary(255), "11111111");
    }

    #[test]
    fn twos_complement_negates() {
        assert_eq!(twos_complement(13), 243);
        assert_eq!(add_u8(13, twos_complement(13)), 0);
    }
}