pub fn is_special_word(xs: &str) -> bool {
    xs == "pneumonoultramicroscopicsilicovolcanoconiosis"
        || xs == "supercalifragilisticexpialidocious"
        || xs == "pseudopseudohypoparathyroidism"
        || xs == "antidisestablishmentarianism"
        || xs == "honorificabilitudinitatibus"
}

/// Checks whether a string represents a number.
pub fn is_number(xs: &str) -> bool {
    u64::from_str_radix(xs, 2)
        .or(u64::from_str_radix(xs, 8))
        .or(u64::from_str_radix(xs, 10))
        .or(u64::from_str_radix(xs, 16))
        .is_ok()
        || xs.parse::<f64>().is_ok()
}

/// Very naive way to check if a string is a valid word.
pub fn is_regular_word(xs: &str) -> bool {
    let mut hyphen = 0;
    let mut quote = false;

    for c in xs.chars() {
        if c == '-' {
            if hyphen == 2 {
                return false;
            } else {
                hyphen += 1;
            }
        } else if c == '\'' {
            if quote {
                return false;
            } else {
                quote = true;
            }
        } else if !c.is_alphabetic() {
            return false;
        }
    }

    true
}

pub fn is_maybe_valid_word(xs: &str) -> bool {
    is_special_word(xs) || is_number(xs) || is_regular_word(xs)
}

pub fn maybe_good_string(xs: &str) -> bool {
    let mut valid_words = 0;
    let mut invalid_words = 0;

    for part in xs.split(' ') {
        if is_maybe_valid_word(part) {
            valid_words += 1;
        } else {
            invalid_words += 1;
        }
    }

    valid_words > invalid_words
}

/// Takes as input a string that has been XOR'd against a single byte (key). It returns
/// a Vec of tuples, where the first element of each tuple is a candidate key and
/// the second is a candidate original string.
pub fn single_byte_xor(input: String) -> Vec<(u8, String)> {
    let bytes = matasano_util::hex_str_to_bytes(input);
    let mut res = Vec::new();

    for byte in 41..=0x7F {
        let key = vec![byte; bytes.len()];
        let xored = matasano_ops::xor_u8(&bytes, key);
        let xored_ascii = String::from_utf8_lossy(&xored);

        if maybe_good_string(xored_ascii.trim()) {
            res.push((byte, xored_ascii.to_string()));
        }
    }

    res
}

pub fn run() {
    println!("====== Single-byte XOR cipher ======");

    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
    for (key, candidate) in single_byte_xor(input) {
        println!("(key: {}) {}", key, candidate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        assert!(is_number("01234567"));
        assert!(is_number("123.456"));
        assert!(!is_number("123.456.789"));
        assert!(is_number("123abc"));
        assert!(!is_number("123abc.456"));
    }
}
