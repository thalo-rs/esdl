use std::str::FromStr;

use nom::{
    bytes::complete::is_not,
    character::complete::{char, space0},
    combinator::{map_res, recognize},
    sequence::{delimited, preceded, tuple},
};
use nom_supreme::tag::complete::tag;
use semver::Version;

use super::{event::Field, IResult, Span};

#[derive(Clone, Debug, PartialEq)]
pub struct CustomType<'i> {
    pub ident: Span<'i>,
    pub fields: Vec<Field<'i>>,
}

pub fn parse_version(input: Span) -> IResult<Span, Version> {
    map_res(
        preceded(
            tuple((tag("version"), space0, char('='), space0)),
            delimited(char('"'), recognize(is_not("\"")), char('"')),
        ),
        Version::from_str,
    )(input)
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use super::parse_version;

    #[test]
    fn version() {
        assert_eq!(
            parse_version(r#"version="0.1.0""#).unwrap(),
            ("", Version::new(0, 1, 0))
        );
        assert_eq!(
            parse_version(r#"version   =  "123.456.789" hello"#).unwrap(),
            (" hello", Version::new(123, 456, 789))
        );
        assert!(parse_version(r#"version="0.1hi0""#).is_err());
    }
}
