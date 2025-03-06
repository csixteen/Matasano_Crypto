pub fn hex_str_to_bytes(xs: String) -> Vec<u8> {
    #[inline]
    fn b_to_dec(b: u8) -> u8 {
        match b {
            48..=57 => b - 48,
            97..=102 => b - 97 + 10,
            65..=70 => b - 65 + 10,
            _ => unreachable!(),
        }
    }

    let mut res = Vec::new();

    for chunk in xs.bytes().collect::<Vec<u8>>().chunks(2) {
        match chunk {
            [a, b] => {
                let aa = b_to_dec(*a);
                let bb = b_to_dec(*b);
                res.push(((aa << 4) & 0xF0) | (bb & 0xF));
            }
            _ => unreachable!(),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case2() {
        assert_eq!(
            vec![
                0x1c, 0x1, 0x11, 0x0, 0x1f, 0x1, 0x1, 0x0, 0x6, 0x1a, 0x2, 0x4b, 0x53, 0x53, 0x50,
                0x9, 0x18, 0x1c
            ],
            hex_str_to_bytes("1c0111001f010100061a024b53535009181c".to_string())
        );
    }
}
