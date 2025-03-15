use std::{fs::File, io::Read, path::Path};

/// Takes a string with hexadecimal characters and returns a Vec with the bytes represented by
/// those characters.
///
/// ```text
/// assert_eq!(
///     vec![0xde, 0xad, 0xbe, 0xef],
///     hex_str_to_bytes("deadbeef")
/// );
/// ```
pub fn hex_str_to_bytes(xs: &str) -> Vec<u8> {
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
    let mut iter = xs.bytes();

    loop {
        let a = iter.next();
        let b = iter.next();

        match (a, b) {
            (None, None) => {
                return res;
            }
            (Some(aa), Some(bb)) => {
                let aaa = b_to_dec(aa);
                let bbb = b_to_dec(bb);
                res.push(((aaa << 4) & 0xF0) | (bbb & 0xF));
            }
            _ => unreachable!(),
        }
    }
}

pub fn get_file_contents(name: impl AsRef<Path>) -> ::std::io::Result<Vec<String>> {
    let mut buffer = String::new();
    let mut file = File::open(name)?;

    file.read_to_string(&mut buffer).unwrap();

    Ok(buffer.trim().split("\n").map(String::from).collect())
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
            hex_str_to_bytes("1c0111001f010100061a024b53535009181c")
        );
    }
}
