use crate::prim::{pred, Parser, Result};

pub fn item(input: &str) -> Result<'_, char> {
    match input.chars().next() {
        Some(c) => Ok((&input[c.len_utf8()..], c)),
        None => Err(input),
    }
}

pub fn ascii_digit<'a>() -> impl Parser<'a, char> {
    pred(item, |c| c.is_ascii_digit())
}

pub fn ascii_hexdigit<'a>() -> impl Parser<'a, char> {
    pred(item, |c| c.is_ascii_hexdigit())
}

pub fn upper_case<'a>() -> impl Parser<'a, char> {
    pred(item, |c| c.is_uppercase())
}

pub fn lower_case<'a>() -> impl Parser<'a, char> {
    pred(item, |c| c.is_uppercase())
}

pub fn whitespace<'a>() -> impl Parser<'a, char> {
    pred(item, |c| c.is_whitespace())
}

pub fn char_<'a>(c: char) -> impl Parser<'a, char> {
    pred(item, move |c2| *c2 == c)
}
