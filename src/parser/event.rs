use nom::{
    character::streaming::space0,
    combinator::map,
    sequence::{separated_pair, tuple},
};
use nom_supreme::tag::complete::tag;

use super::{
    ident::parse_snake_ident,
    parsers::keyword_ident_structure,
    types::{parse_type, Type},
    IResult, Span,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Event<'i> {
    pub ident: Span<'i>,
    pub fields: Vec<Field<'i>>,
}

pub fn parse_event(input: Span) -> IResult<Span, Event<'_>> {
    map(keyword_ident_structure("event"), |(ident, fields)| Event {
        ident,
        fields,
    })(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field<'i> {
    pub ident: Span<'i>,
    pub ty: Type<'i>,
}

pub fn parse_field(input: Span) -> IResult<Span, Field<'_>> {
    map(
        separated_pair(
            parse_snake_ident,
            tuple((space0, tag(":"), space0)),
            parse_type,
        ),
        |(ident, ty)| Field { ident, ty },
    )(input)
}
