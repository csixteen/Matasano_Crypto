//--------------------------------------------------------------------------
// BEGIN NOTE
//
// These methods will soon be replaced by matasano-parser. I'm using
// them for now to have a quick (and naive) way of deciding if a string
// is garbage or not. It worked well for Challenge 3, but not so well for
// Challenge 4 (there were 2 false positives).
fn is_special_word(xs: &str) -> bool {
    xs == "pneumonoultramicroscopicsilicovolcanoconiosis"
        || xs == "supercalifragilisticexpialidocious"
        || xs == "pseudopseudohypoparathyroidism"
        || xs == "antidisestablishmentarianism"
        || xs == "honorificabilitudinitatibus"
}

/// Checks whether a string represents a number.
fn is_number(xs: &str) -> bool {
    u64::from_str_radix(xs, 2)
        .or(u64::from_str_radix(xs, 8))
        .or(xs.parse::<u64>())
        .or(u64::from_str_radix(xs, 16))
        .is_ok()
        || xs.parse::<f64>().is_ok()
}

/// Very naive way to check if a string is a valid word.
fn is_regular_word(xs: &str) -> bool {
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

fn is_maybe_valid_word(xs: &str) -> bool {
    is_special_word(xs) || is_number(xs) || is_regular_word(xs)
}

fn maybe_good_string(xs: &str) -> bool {
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
// END NOTE
//--------------------------------------------------------------------

/// Takes as input a string that has been XOR'd against a single byte (key). It returns
/// a Vec of tuples, where the first element of each tuple is a candidate key and
/// the second is a candidate original string.
pub fn single_byte_xor(input: &str) -> Vec<(u8, String)> {
    let bytes = matasano_util::hex_str_to_bytes(input);
    let mut res = Vec::new();

    for byte in 41..=0x7F {
        let key = vec![byte; bytes.len()];
        let xored = matasano_bitwise::xor(&bytes, key);
        let xored_ascii = String::from_utf8_lossy(&xored);

        if maybe_good_string(xored_ascii.trim()) {
            res.push((byte, xored_ascii.trim().to_string()));
        }
    }

    res
}

pub fn detect_single_char_xor() -> anyhow::Result<()> {
    let data = matasano_util::get_file_contents("./data/4.txt")?;

    // Need to improve the algorithm for detecting valid sentences. There are two false positives.
    // The correct sentence is also printed: "Now that the party is jumping"
    for line in data {
        for (key, candidate) in single_byte_xor(&line) {
            println!("(key: {}) {}", key, candidate);
        }
    }

    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    println!("====== 3. Single-byte XOR cipher ======");

    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    for (key, candidate) in single_byte_xor(input) {
        println!("(key: {}) {}", key, candidate);
    }

    println!("====== 4. Detect single-character XOR ======");
    detect_single_char_xor()?;

    Ok(())
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
