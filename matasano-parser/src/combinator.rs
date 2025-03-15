use crate::prim::{map, Parser};

pub fn pair<'a, P1, P2, R1, R2>(left: P1, right: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input: &'a str| {
        left.parse(input).and_then(|(next_input, result1)| {
            right
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

pub fn left<'a, P1, P2, R1, R2>(left: P1, right: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(left, right), |(l, _r)| l)
}

pub fn right<'a, P1, P2, R1, R2>(left: P1, right: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(left, right), |(_l, r)| r)
}

pub fn either<'a, P1, P2, A>(left: P1, right: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input: &'a str| match left.parse(input) {
        l @ Ok(_) => l,
        Err(_) => right.parse(input),
    }
}

pub fn many1<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
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

pub fn skip_many1<'a, P, A>(parser: P) -> impl Parser<'a, ()>
where
    P: Parser<'a, A>,
{
    move |mut input: &'a str| {
        if let Ok((next_input, _output)) = parser.parse(input) {
            input = next_input;
        } else {
            return Err(input);
        }

        while let Ok((next_input, _output)) = parser.parse(input) {
            input = next_input;
        }

        Ok((input, ()))
    }
}

pub fn match_literal<'b, 'a: 'b>(expected: &'b str) -> impl Parser<'a, ()> + 'b {
    move |input: &'a str| {
        if let Some(stripped) = input.strip_prefix(expected) {
            Ok((stripped, ()))
        } else {
            Err(input)
        }
    }
}
