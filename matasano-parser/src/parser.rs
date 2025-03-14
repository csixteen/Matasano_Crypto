use crate::core::Parser;

pub fn match_literal<'b, 'a: 'b>(expected: &'b str) -> impl Parser<'a, ()> + 'b {
    move |input: &'a str| {
        if let Some(stripped) = input.strip_prefix(expected) {
            Ok((stripped, ()))
        } else {
            Err(input)
        }
    }
}

pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input: &'a str| {
        let mut res = Vec::new();

        if let Ok((next_input, next_output)) = parser.parse(input) {
            input = next_input;
            res.push(next_output);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_output)) = parser.parse(input) {
            input = next_input;
            res.push(next_output);
        }

        Ok((input, res))
    }
}

pub fn zero_or_move<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input: &'a str| {
        let mut res = Vec::new();

        while let Ok((next_input, next_output)) = parser.parse(input) {
            input = next_input;
            res.push(next_output);
        }

        Ok((input, res))
    }
}
