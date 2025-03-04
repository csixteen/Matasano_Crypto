pub fn encode(input: impl AsRef<[u8]>) -> Vec<u8> {
    let input = input.as_ref();
    let len = input.len();
    // The way Base64 works means that for every 3 bytes of input there are 4 Base64 characters (each of them 6 bits).
    let l = {
        if len % 3 == 0 {
            len
        } else {
            (len / 3 + 1) * 3
        }
    };
    let mut res = Vec::with_capacity(l * 4);
    let mut i = 0;
    while i < len {
        res.push(TO_ASCII[((input[i] & 0xFC) >> 2) as usize]);
        if i + 2 < len {
            res.push(TO_ASCII[(((input[i] & 0x3) << 4) | ((input[i + 1] & 0xF0) >> 4)) as usize]);
            res.push(
                TO_ASCII[(((input[i + 1] & 0xF) << 2) | ((input[i + 2] & 0xC0) >> 6)) as usize],
            );
            res.push(TO_ASCII[(input[i + 2] & 0x3F) as usize]);
        } else if i + 1 < len {
            res.push(TO_ASCII[(((input[i] & 0x3) << 4) | ((input[i + 1] & 0xF0) >> 4)) as usize]);
            res.push(TO_ASCII[((input[i + 1] & 0xF) << 2) as usize]);
            res.push(PADDING);
        } else {
            res.push(TO_ASCII[((input[i] & 0x3) << 4) as usize]);
            res.push(PADDING);
            res.push(PADDING);
        }

        i += 3;
    }

    res
}

pub fn decode(input: impl AsRef<str>) -> Vec<u8> {
    let input = input.as_ref().chars().collect::<Vec<_>>();
    let len = input.len() * 6 / 8;
    let mut res = Vec::with_capacity(len);
    for chunk in input.chunks(4) {
        match chunk {
            [a, b, '=', '='] => {
                let x = from_ascii(*a) & 0x3F;
                let y = from_ascii(*b) & 0x30;
                res.push((x << 2) | (y >> 4));
            }
            [a, b, c, '='] => {
                let x = from_ascii(*a) & 0x3F;
                let y = from_ascii(*b) & 0x3F;
                let z = from_ascii(*c) & 0x3C;
                res.push((x << 2) | (y >> 4));
                res.push(((y & 0xF) << 4) | ((z & 0x3C) >> 2));
            }
            [a, b, c, d] => {
                let x = from_ascii(*a) & 0x3F;
                let y = from_ascii(*b) & 0x3F;
                let z = from_ascii(*c) & 0x3F;
                let w = from_ascii(*d) & 0x3F;
                res.push((x << 2) | (y >> 4));
                res.push(((y & 0xF) << 4) | ((z & 0x3C) >> 2));
                res.push((z << 6) | w);
            }
            _ => unreachable!(),
        }
    }

    res
}

const fn from_ascii(c: char) -> u8 {
    match c {
        'A'..='Z' => c as u8 - 0x41,
        'a'..='z' => c as u8 - 0x47,
        '0'..='9' => c as u8 + 0x4,
        '+' => 0x3E,
        '/' => 0x3F,
        _ => unreachable!(),
    }
}

const TO_ASCII: [u8; 64] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7A, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2B, 0x2F,
];
const PADDING: u8 = 0x3D;

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<u8> {
        let mut res = Vec::new();

        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
        for chunk in input.chars().collect::<Vec<char>>().chunks(2) {
            match chunk {
                [a, b] => {
                    let mut s = String::new();
                    s.push(*a);
                    s.push(*b);
                    res.push(u8::from_str_radix(&s, 16).unwrap());
                }
                _ => unreachable!("Shouldn't happen"),
            }
        }

        res
    }

    #[test]
    fn test_encode1() {
        assert_eq!(
            "TWFu".to_string(),
            String::from_utf8_lossy(&encode([0x4D, 0x61, 0x6E]))
        );
    }

    #[test]
    fn test_encode_padding1() {
        assert_eq!(
            "TWE=".to_string(),
            String::from_utf8_lossy(&encode([0x4D, 0x61]))
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!("wor".to_string(), String::from_utf8_lossy(&decode("d29y")));
    }

    #[test]
    fn test_decode_padding1() {
        assert_eq!("wo".to_string(), String::from_utf8_lossy(&decode("d28=")));
    }

    #[test]
    fn test_decode_padding2() {
        assert_eq!("w".to_string(), String::from_utf8_lossy(&decode("dw==")));
    }

    #[test]
    fn test_case1() {
        let expected =
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
        assert_eq!(expected, String::from_utf8_lossy(&encode(example_input())));
    }

    #[test]
    fn test_case2() {
        let input = "Many hands make light work.".to_string();
        let expected = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".to_string();
        assert_eq!(expected, String::from_utf8_lossy(&encode(input)));
    }

    #[test]
    fn test_roundtrip() {
        let input = "Many hands make light work.".to_string();
        assert_eq!(
            input.clone(),
            String::from_utf8_lossy(&decode(String::from_utf8_lossy(&encode(input))))
        );
    }
}
