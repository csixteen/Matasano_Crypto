use crate::{
    combinator::{left, skip_many1},
    prim::{pred, Parser, Result},
};

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

pub fn char_<'a>(c: char) -> impl Parser<'a, char> {
    pred(item, move |c2| *c2 == c)
}

// ----------------- white space and symbols -----------------

pub fn single_space<'a>() -> impl Parser<'a, char> {
    pred(item, |c| c.is_whitespace())
}

pub fn white_space<'a>() -> impl Parser<'a, ()> {
    skip_many1(single_space())
}

pub fn lexeme<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    left(parser, white_space())
}

#[cfg(test)]
mod tests {
    use crate::{combinator::between, prim::many};

    use super::*;

    #[test]
    fn test_many_predicate() {
        let parser = many(ascii_digit());
        assert_eq!(Ok(("", vec!['1', '2', '3'])), parser.parse("123"));
        assert_eq!(Ok(("abc", Vec::new())), parser.parse("abc"));
    }

    #[test]
    fn test_many_between() {
        let parser = between(char_('"'), many(ascii_digit()), char_('"'));
        assert_eq!(Ok(("", vec!['1', '2', '3'])), parser.parse("\"123\""));
    }
}
