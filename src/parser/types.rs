use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::{map, opt, value},
    sequence::{delimited, pair},
};
use nom_supreme::tag::complete::tag;

use super::{
    event::Field, ident::parse_camel_ident, parsers::keyword_ident_structure, IResult, Span,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomType<'i> {
    pub ident: Span<'i>,
    pub fields: Vec<Field<'i>>,
}

pub fn parse_custom_type(input: Span) -> IResult<Span, CustomType<'_>> {
    map(keyword_ident_structure("type"), |(ident, fields)| {
        CustomType { ident, fields }
    })(input)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<'i> {
    Single(OptionalOrRequiredType<'i>),
    Array {
        inner: OptionalOrRequiredType<'i>,
        optional: bool,
    },
}

pub fn parse_type(input: Span) -> IResult<Span, Type<'_>> {
    let single_type_parser = map(parse_optional_or_required_type, Type::Single);
    let array_type_parser = map(
        pair(
            delimited(
                pair(tag("["), space0),
                parse_optional_or_required_type,
                pair(space0, tag("]")),
            ),
            opt(char('?')),
        ),
        |(ty, optional)| Type::Array {
            inner: ty,
            optional: optional.is_some(),
        },
    );
    alt((single_type_parser, array_type_parser))(input)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalarOrUserType<'i> {
    Scalar(Scalar),
    UserDefined(Span<'i>),
}

pub fn parse_scalar_or_user_type(input: Span) -> IResult<Span, ScalarOrUserType<'_>> {
    let scalar_parser = map(parse_scalar, ScalarOrUserType::Scalar);
    let user_defined_parser = map(parse_camel_ident, |ident| {
        ScalarOrUserType::UserDefined(ident)
    });

    alt((scalar_parser, user_defined_parser))(input)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionalOrRequiredType<'i> {
    Optional(ScalarOrUserType<'i>),
    Required(ScalarOrUserType<'i>),
}

pub fn parse_optional_or_required_type(input: Span) -> IResult<Span, OptionalOrRequiredType<'_>> {
    let mut optional_or_required_parser = map(
        pair(parse_scalar_or_user_type, opt(char('?'))),
        |(scalar_or_user_type, optional)| {
            if optional.is_some() {
                OptionalOrRequiredType::Optional(scalar_or_user_type)
            } else {
                OptionalOrRequiredType::Required(scalar_or_user_type)
            }
        },
    );

    optional_or_required_parser(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scalar {
    String,
    /// 32-bit signed integer
    Int,
    /// 64-bit signed integer
    Long,
    /// 32-bit float
    Float,
    /// 64-bit float
    Double,
    /// Binary value
    Bool,
    /// Sequence of 8-bit unsigned bytes
    Bytes,
}

pub fn parse_scalar_string(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::String, tag("String"))(input)
}

pub fn parse_scalar_int(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Int, tag("Int"))(input)
}

pub fn parse_scalar_uint(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Long, tag("Long"))(input)
}

pub fn parse_scalar_float(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Float, tag("Float"))(input)
}

pub fn parse_scalar_double(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Double, tag("Double"))(input)
}

pub fn parse_scalar_bool(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Bool, tag("Bool"))(input)
}

pub fn parse_scalar_bytes(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Bytes, tag("Bytes"))(input)
}

pub fn parse_scalar(input: Span) -> IResult<Span, Scalar> {
    alt((
        parse_scalar_string,
        parse_scalar_int,
        parse_scalar_uint,
        parse_scalar_float,
        parse_scalar_double,
        parse_scalar_bool,
        parse_scalar_bytes,
    ))(input)
}
