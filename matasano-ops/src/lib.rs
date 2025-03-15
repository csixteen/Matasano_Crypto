use std::ops::{BitAnd, BitOr, BitXor};

use matasano_util::hex_str_to_bytes;
use paste::paste;

macro_rules! boolean_ops {
    ( $( $name:ident => $op:ident ),* ) => {$(
        paste! {
            pub fn [<$name _str>](a: String, b: String) -> Vec<u8> {
                let a = hex_str_to_bytes(a);
                let b = hex_str_to_bytes(b);
                $name(a, b)
            }

            pub fn $name(a: impl AsRef<[u8]>, b: impl AsRef<[u8]>) -> Vec<u8> {
                let a = a.as_ref();
                let b = b.as_ref();
                a.iter().zip(b.iter()).fold(Vec::new(), |mut acc, (x, y)| {
                    acc.push(u8::$op(*x, *y));
                    acc
                })
            }

            pub fn [<$name _mut>](mut a: impl AsMut<[u8]>, b: impl AsRef<[u8]>) {
                let a = a.as_mut();
                let b = b.as_ref();
                let mut i = 0;

                while i < a.len() && i < b.len() {
                    a[i] = u8::$op(a[i], b[i]);
                    i += 1;
                }
            }
        }
    )*};
}

boolean_ops!(xor => bitxor, or => bitor, and => bitand);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(
            vec![
                0x74, 0x68, 0x65, 0x20, 0x6B, 0x69, 0x64, 0x20, 0x64, 0x6F, 0x6E, 0x27, 0x74, 0x20,
                0x70, 0x6C, 0x61, 0x79
            ],
            xor_str(
                "1c0111001f010100061a024b53535009181c".to_string(),
                "686974207468652062756c6c277320657965".to_string()
            )
        );
    }
}
