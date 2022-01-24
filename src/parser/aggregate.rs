use nom::{
    branch::alt,
    character::complete::{char, multispace0, space0, space1},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
};
use nom_supreme::tag::complete::tag;

use super::{
    ident::{parse_camel_ident, parse_snake_ident},
    parsers::delimited_multiline_list0,
    types::{parse_type, Type},
    IResult, Span,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Aggregate<'i> {
    pub ident: Span<'i>,
    pub commands: Vec<Command<'i>>,
}

pub fn parse_aggregate(input: Span) -> IResult<Span, Aggregate> {
    let ident_parser = preceded(pair(tag("aggregate"), space1), parse_camel_ident);
    let commands_parser = delimited_multiline_list0(|| tag("{"), parse_command, || tag("}"));
    // let commands_parser = delimited(
    //     tuple((space0, tag("{"), space0, newline, multispace0)),
    //     separated_list0(tuple((multispace0, multispace1, space0)), parse_command),
    //     tuple((space0, multispace1, space0, tag("}"))),
    // );

    map(
        separated_pair(ident_parser, multispace0, commands_parser),
        |(ident, commands)| Aggregate { ident, commands },
    )(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Command<'i> {
    pub ident: Span<'i>,
    pub params: Vec<Param<'i>>,
    pub return_type: ReturnType<'i>,
}

pub fn parse_command(input: Span) -> IResult<Span, Command<'_>> {
    let ident_parser = parse_snake_ident;
    let params_parser = parse_params;
    let ident_params_parser = pair(
        separated_pair(ident_parser, multispace0, params_parser),
        preceded(
            tuple((multispace0, tag(":"), multispace0)),
            parse_return_type,
        ),
    );
    map(ident_params_parser, |((ident, params), return_type)| {
        Command {
            ident,
            params,
            return_type,
        }
    })(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Param<'i> {
    pub ident: Span<'i>,
    pub ty: Type<'i>,
}

pub fn parse_params(input: Span) -> IResult<Span, Vec<Param<'_>>> {
    delimited(
        tuple((tag("("), multispace0)),
        separated_list0(tuple((multispace0, tag(","), multispace0)), parse_param),
        tuple((multispace0, tag(")"))),
    )(input)
}

pub fn parse_param(input: Span) -> IResult<Span, Param<'_>> {
    map(
        separated_pair(
            parse_snake_ident,
            tuple((space0, tag(":"), space0)),
            parse_type,
        ),
        |(ident, ty)| Param { ident, ty },
    )(input)
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnType<'i> {
    Single(ReturnTypeOptionalOrRequired<'i>),
    Tuple(Vec<ReturnTypeOptionalOrRequired<'i>>),
}

pub fn parse_return_type(input: Span) -> IResult<Span, ReturnType<'_>> {
    let single_return_type_parse = map(parse_return_type_optional_or_required, ReturnType::Single);
    let tuple_return_type_parse = map(
        delimited(
            pair(tag("("), multispace0),
            separated_list0(
                tuple((multispace0, tag("|"), multispace0)),
                parse_return_type_optional_or_required,
            ),
            pair(multispace0, tag(")")),
        ),
        ReturnType::Tuple,
    );

    alt((single_return_type_parse, tuple_return_type_parse))(input)
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnTypeOptionalOrRequired<'i> {
    Optional(Span<'i>),
    Required(Span<'i>),
}

pub fn parse_return_type_optional_or_required(
    input: Span,
) -> IResult<Span, ReturnTypeOptionalOrRequired<'_>> {
    map(pair(parse_camel_ident, opt(char('!'))), |(ty, required)| {
        if required.is_some() {
            ReturnTypeOptionalOrRequired::Required(ty)
        } else {
            ReturnTypeOptionalOrRequired::Optional(ty)
        }
    })(input)
}
