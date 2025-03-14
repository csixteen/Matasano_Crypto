pub type Result<'a, T> = ::std::result::Result<(&'a str, T), &'a str>;

pub trait Parser<'a, Out> {
    fn parse(&self, input: &'a str) -> Result<'a, Out>;

    fn map<F, NewOut>(self, map_fn: F) -> BoxedParser<'a, NewOut>
    where
        Self: Sized + 'a,
        Out: 'a,
        NewOut: 'a,
        F: Fn(Out) -> NewOut + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn fmap<F, NextParser, NextOut>(self, f: F) -> BoxedParser<'a, NextOut>
    where
        Self: Sized + 'a,
        Out: 'a,
        NextOut: 'a,
        NextParser: Parser<'a, NextOut> + 'a,
        F: Fn(Out) -> NextParser + 'a,
    {
        BoxedParser::new(fmap(self, f))
    }

    fn pred<F>(self, f: F) -> BoxedParser<'a, Out>
    where
        Self: Sized + 'a,
        Out: 'a,
        F: Fn(&Out) -> bool + 'a,
    {
        BoxedParser::new(pred(self, f))
    }
}

impl<'a, F, Out> Parser<'a, Out> for F
where
    F: Fn(&'a str) -> Result<'a, Out>,
{
    fn parse(&self, input: &'a str) -> Result<'a, Out> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    pub fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> Result<'a, Output> {
        self.parser.parse(input)
    }
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input: &'a str| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub fn fmap<'a, P, NextP, F, A, B>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input: &'a str| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(error) => Err(error),
    }
}

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

pub fn pred<'a, P, A, F>(parser: P, f: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input: &'a str| {
        if let Ok((next_input, result)) = parser.parse(input) {
            if f(&result) {
                return Ok((next_input, result));
            }
        }
        Err(input)
    }
}
