use crate::prim::Parser;

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
