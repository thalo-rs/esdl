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

#[derive(Clone, Debug, PartialEq)]
pub struct CustomType<'i> {
    pub ident: Span<'i>,
    pub fields: Vec<Field<'i>>,
}

pub fn parse_custom_type(input: Span) -> IResult<Span, CustomType<'_>> {
    map(keyword_ident_structure("type"), |(ident, fields)| {
        CustomType { ident, fields }
    })(input)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type<'i> {
    Single(OptionalOrRequiredType<'i>),
    Array {
        inner: OptionalOrRequiredType<'i>,
        required: bool,
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
            opt(char('!')),
        ),
        |(ty, required)| Type::Array {
            inner: ty,
            required: required.is_some(),
        },
    );
    alt((single_type_parser, array_type_parser))(input)
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum OptionalOrRequiredType<'i> {
    Optional(ScalarOrUserType<'i>),
    Required(ScalarOrUserType<'i>),
}

pub fn parse_optional_or_required_type(input: Span) -> IResult<Span, OptionalOrRequiredType<'_>> {
    let mut optional_or_required_parser = map(
        pair(parse_scalar_or_user_type, opt(char('!'))),
        |(scalar_or_user_type, required)| {
            if required.is_some() {
                OptionalOrRequiredType::Required(scalar_or_user_type)
            } else {
                OptionalOrRequiredType::Optional(scalar_or_user_type)
            }
        },
    );

    optional_or_required_parser(input)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scalar {
    String,
    Int,
    Float,
    Bool,
    Timestamp,
}

pub fn parse_scalar_string(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::String, tag("String"))(input)
}

pub fn parse_scalar_int(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Int, tag("Int"))(input)
}

pub fn parse_scalar_float(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Float, tag("Float"))(input)
}

pub fn parse_scalar_bool(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Bool, tag("Bool"))(input)
}

pub fn parse_scalar_timestamp(input: Span) -> IResult<Span, Scalar> {
    value(Scalar::Timestamp, tag("Timestamp"))(input)
}

pub fn parse_scalar(input: Span) -> IResult<Span, Scalar> {
    alt((
        parse_scalar_string,
        parse_scalar_int,
        parse_scalar_float,
        parse_scalar_bool,
        parse_scalar_timestamp,
    ))(input)
}

// #[cfg(test)]
// mod tests {
//     use super::{parse_type, OptionalOrRequiredType, Scalar, ScalarOrUserType, Type};

//     #[test]
//     fn it_parses_type() {
//         assert_eq!(
//             parse_type("String").unwrap(),
//             (
//                 "",
//                 Type::Single(OptionalOrRequiredType::Optional(ScalarOrUserType::Scalar(
//                     Scalar::String
//                 )))
//             )
//         );
//         assert_eq!(
//             parse_type("Int").unwrap(),
//             (
//                 "",
//                 Type::Single(OptionalOrRequiredType::Optional(ScalarOrUserType::Scalar(
//                     Scalar::Int
//                 )))
//             )
//         );
//         assert_eq!(
//             parse_type("Float").unwrap(),
//             (
//                 "",
//                 Type::Single(OptionalOrRequiredType::Optional(ScalarOrUserType::Scalar(
//                     Scalar::Float
//                 )))
//             )
//         );
//         assert_eq!(
//             parse_type("Bool").unwrap(),
//             (
//                 "",
//                 Type::Single(OptionalOrRequiredType::Optional(ScalarOrUserType::Scalar(
//                     Scalar::Bool
//                 )))
//             )
//         );
//         assert_eq!(
//             parse_type("MyType").unwrap(),
//             (
//                 "",
//                 Type::Single(OptionalOrRequiredType::Optional(
//                     ScalarOrUserType::UserDefined("MyType")
//                 ))
//             )
//         );
//         assert_eq!(
//             parse_type("String!").unwrap(),
//             (
//                 "",
//                 Type::Single(OptionalOrRequiredType::Required(ScalarOrUserType::Scalar(
//                     Scalar::String
//                 )))
//             )
//         );
//         assert_eq!(
//             parse_type("MyType!").unwrap(),
//             (
//                 "",
//                 Type::Single(OptionalOrRequiredType::Required(
//                     ScalarOrUserType::UserDefined("MyType")
//                 ))
//             )
//         );
//         assert_eq!(
//             parse_type("[String]").unwrap(),
//             (
//                 "",
//                 Type::Array {
//                     inner: OptionalOrRequiredType::Optional(ScalarOrUserType::Scalar(
//                         Scalar::String
//                     )),
//                     required: false
//                 }
//             )
//         );
//         assert_eq!(
//             parse_type("[String]!").unwrap(),
//             (
//                 "",
//                 Type::Array {
//                     inner: OptionalOrRequiredType::Optional(ScalarOrUserType::Scalar(
//                         Scalar::String
//                     )),
//                     required: true
//                 }
//             )
//         );
//         assert_eq!(
//             parse_type("[String!]").unwrap(),
//             (
//                 "",
//                 Type::Array {
//                     inner: OptionalOrRequiredType::Required(ScalarOrUserType::Scalar(
//                         Scalar::String
//                     )),
//                     required: false
//                 }
//             )
//         );
//         assert_eq!(
//             parse_type("[String!]!").unwrap(),
//             (
//                 "",
//                 Type::Array {
//                     inner: OptionalOrRequiredType::Required(ScalarOrUserType::Scalar(
//                         Scalar::String
//                     )),
//                     required: true
//                 }
//             )
//         );
//         assert_eq!(
//             parse_type("[MyType]").unwrap(),
//             (
//                 "",
//                 Type::Array {
//                     inner: OptionalOrRequiredType::Optional(ScalarOrUserType::UserDefined(
//                         "MyType"
//                     )),
//                     required: false
//                 }
//             )
//         );
//         assert_eq!(
//             parse_type("[MyType]!").unwrap(),
//             (
//                 "",
//                 Type::Array {
//                     inner: OptionalOrRequiredType::Optional(ScalarOrUserType::UserDefined(
//                         "MyType"
//                     )),
//                     required: true
//                 }
//             )
//         );
//         assert_eq!(
//             parse_type("[MyType!]").unwrap(),
//             (
//                 "",
//                 Type::Array {
//                     inner: OptionalOrRequiredType::Required(ScalarOrUserType::UserDefined(
//                         "MyType"
//                     )),
//                     required: false
//                 }
//             )
//         );
//         assert_eq!(
//             parse_type("[MyType!]!").unwrap(),
//             (
//                 "",
//                 Type::Array {
//                     inner: OptionalOrRequiredType::Required(ScalarOrUserType::UserDefined(
//                         "MyType"
//                     )),
//                     required: true
//                 }
//             )
//         );
//     }
// }
