use std::ops::RangeFrom;

use nom::{
    branch::alt,
    character::complete::{char, multispace0, newline, space0, space1},
    combinator::map,
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    AsChar, IResult, InputIter, InputLength, InputTakeAtPosition, Parser, Slice,
};
use nom_supreme::tag::complete::tag;

use super::{
    event::{parse_field, Field},
    ident::parse_camel_ident,
    Span,
};

pub fn multinewline1<T, E: ParseError<T>>(input: T) -> IResult<T, (T, char, T), E>
where
    T: InputTakeAtPosition + InputIter + Slice<RangeFrom<usize>> + Clone,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
    <T as InputIter>::Item: AsChar + Clone,
{
    tuple((space0, newline, multispace0))(input)
}

pub fn delimited_multiline_list0<I, O1, O2, O3, E: ParseError<I>, F, G, H>(
    first: impl Fn() -> F,
    second: G,
    third: impl Fn() -> H,
) -> impl FnMut(I) -> IResult<I, Vec<O2>, E>
where
    I: InputTakeAtPosition + InputIter + InputLength + Slice<RangeFrom<usize>> + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    <I as InputIter>::Item: AsChar + Clone,
    F: Parser<I, O1, E>,
    G: Parser<I, O2, E>,
    H: Parser<I, O3, E>,
{
    alt((
        map(tuple((first(), multispace0, third())), |_| Vec::new()),
        delimited(
            pair(first(), multinewline1),
            separated_list0(multinewline1, second),
            pair(multinewline1, third()),
        ),
    ))
}

pub fn keyword_ident_structure<'i>(
    keyword: &'static str,
) -> impl FnMut(Span<'i>) -> super::IResult<Span<'i>, (Span<'i>, Vec<Field<'i>>)> {
    let ident_parser = preceded(pair(tag(keyword), space1), parse_camel_ident);
    let fields_parser = delimited_multiline_list0(|| char('{'), parse_field, || char('}'));

    separated_pair(ident_parser, multispace0, fields_parser)
}
