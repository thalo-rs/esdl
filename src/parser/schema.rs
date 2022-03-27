use nom::{
    branch::alt,
    character::complete::{multispace0, multispace1},
    combinator::{map, value},
    multi::many0,
    sequence::delimited,
};
use semver::Version;

use super::{
    aggregate::{parse_aggregate, Aggregate},
    event::{parse_event, Event},
    types::{parse_custom_type, CustomType},
    version::parse_version,
    IResult, Span,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Schema<'i> {
    pub versions: Vec<Version>,
    pub aggregates: Vec<Aggregate<'i>>,
    pub events: Vec<Event<'i>>,
    pub types: Vec<CustomType<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
enum SchemaType<'i> {
    Aggregate(Aggregate<'i>),
    Event(Event<'i>),
    CustomType(CustomType<'i>),
    Version(Version),
    Noop,
}

pub fn parse_schema(input: Span) -> IResult<Span, Schema> {
    let (tail, schema_types) = delimited(
        multispace0,
        many0(alt((
            value(SchemaType::Noop, multispace1),
            map(parse_aggregate, SchemaType::Aggregate),
            map(parse_version, SchemaType::Version),
            map(parse_event, SchemaType::Event),
            map(parse_custom_type, SchemaType::CustomType),
        ))),
        multispace0,
    )(input)?;

    let schema = schema_types
        .into_iter()
        .fold(Schema::default(), |mut acc, item| {
            match item {
                SchemaType::Aggregate(aggregate) => acc.aggregates.push(aggregate),
                SchemaType::Event(event) => acc.events.push(event),
                SchemaType::CustomType(ty) => acc.types.push(ty),
                SchemaType::Version(version) => acc.versions.push(version),
                SchemaType::Noop => {}
            }

            acc
        });

    Ok((tail, schema))
}
