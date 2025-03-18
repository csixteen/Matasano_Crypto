use crate::prim::Parser;

/// Parser that succeeds if the current character is in the supplied list of
/// characters `xs`. Returns the parsed character.
pub fn one_of<'a>(xs: impl AsRef<str>) -> impl Parser<'a, char> {
    move |input: &'a str| {
        let xs = xs.as_ref();
        if let Some(c) = input.chars().next() {
            for x in xs.chars() {
                if x == c {
                    return Ok((&input[c.len_utf8()..], c));
                }
            }
            Err(input)
        } else {
            Err("empty string")
        }
    }
}

/// As the dual of `one_of`, `none_of` succeeds if the current character of a non-empty
/// input doesn't match any in the supplied list of characters. Returns the parsed
/// character.
pub fn none_of<'a>(xs: impl AsRef<str>) -> impl Parser<'a, char> {
    move |input: &'a str| {
        let xs = xs.as_ref();
        if let Some(c) = input.chars().next() {
            for x in xs.chars() {
                if x == c {
                    return Err(input);
                }
            }
            Ok((&input[c.len_utf8()..], c))
        } else {
            Err("empty string")
        }
    }
}

/// Parses a sequence of characters given by `s`. Returns the
/// parsed string.
pub fn string<'a>(s: &'a str) -> impl Parser<'a, String> {
    move |input: &'a str| {
        if let Some(stripped) = input.strip_prefix(s) {
            Ok((stripped, s.to_owned()))
        } else {
            Err(input)
        }
    }
}

/// Parses a sequence of characters given by `s`. Returns the
/// parsed string, but doesn't consume the matching prefix.
pub fn string_<'a>(s: &'a str) -> impl Parser<'a, String> {
    move |input: &'a str| {
        if input.starts_with(s) {
            Ok((input, s.to_owned()))
        } else {
            Err(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_of() {
        let parser = one_of("aeiou");
        assert_eq!(Ok(("bc123", 'a')), parser.parse("abc123"));
        assert_eq!(Err("xyz123"), parser.parse("xyz123"));
    }

    #[test]
    fn test_none_of() {
        let parser = none_of("aeiou");
        assert_eq!(Ok(("bc123", 'w')), parser.parse("wbc123"));
        assert_eq!(Err("abc123"), parser.parse("abc123"));
    }
}
