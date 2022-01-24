use nom::{
    branch::alt,
    character::complete::{alphanumeric1, satisfy},
    combinator::recognize,
    multi::many0,
    sequence::pair,
};
use nom_supreme::tag::complete::tag;

use super::{IResult, Span};

pub fn parse_camel_ident(input: Span) -> IResult<Span, Span> {
    recognize(pair(
        satisfy(|c: char| c.is_alphabetic() && c.is_uppercase()),
        many0(alphanumeric1),
    ))(input)
}

pub fn parse_snake_ident(input: Span) -> IResult<Span, Span> {
    recognize(pair(
        alt((
            recognize(satisfy(|c| c.is_alphabetic() && c.is_lowercase())),
            tag("_"),
        )),
        many0(alt((
            recognize(satisfy(|c| {
                if c.is_alphabetic() {
                    c.is_lowercase()
                } else {
                    c.is_numeric()
                }
            })),
            tag("_"),
        ))),
    ))(input)
}

// #[cfg(test)]
// mod tests {
//     use crate::Span;

//     use super::{parse_camel_ident, parse_snake_ident};

//     #[test]
//     fn it_parses_snake_idents() {
//         assert_eq!(parse_snake_ident("hello").unwrap(), ("", "hello"));
//         assert_eq!(
//             parse_snake_ident("hello_world").unwrap(),
//             ("", "hello_world")
//         );
//         assert_eq!(parse_snake_ident("_hello").unwrap(), ("", "_hello"));
//         assert_eq!(
//             parse_snake_ident("_hello_world").unwrap(),
//             ("", "_hello_world")
//         );
//         assert_eq!(parse_snake_ident("hello123").unwrap(), ("", "hello123"));
//         assert_eq!(parse_snake_ident("hello_123").unwrap(), ("", "hello_123"));
//         assert_eq!(parse_snake_ident("helloWorld").unwrap(), ("World", "hello"));
//         assert_eq!(
//             parse_snake_ident("hello_World").unwrap(),
//             ("World", "hello_")
//         );
//         assert!(parse_snake_ident("Hello").is_err());
//         assert!(parse_snake_ident("9Hey").is_err());
//     }

//     #[test]
//     fn it_parses_camel_idents() {
//         // Starts with capital letter
//         assert_eq!(
//             parse_camel_ident(Span::new("Hello")).unwrap(),
//             (Span::new("Hello"), Span::new("Hello"))
//         );

//         // // Has all capitals
//         // assert_eq!(parse_camel_ident("HelloWorld").unwrap(), ("", "HelloWorld"));

//         // // Starts with lowercase
//         // assert!(parse_camel_ident("hello").is_err());

//         // // Starts with _
//         // assert!(parse_camel_ident("_Hello").is_err());

//         // // Does not contain _
//         // assert_eq!(parse_camel_ident("Hello_").unwrap(), ("_", "Hello"));

//         // // Starts with number
//         // assert!(parse_camel_ident("9Hello").is_err());

//         // // Contains number
//         // assert_eq!(
//         //     parse_camel_ident("Hello9World").unwrap(),
//         //     ("", "Hello9World")
//         // );
//     }
// }
