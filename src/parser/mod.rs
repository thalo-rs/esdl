use nom_supreme::{error::ErrorTree, final_parser::final_parser};

pub mod aggregate;
pub mod event;
pub mod ident;
pub mod parsers;
pub mod schema;
pub mod types;

pub type Span<'i> = &'i str;
pub type Error<I> = ErrorTree<I>;
pub type IResult<I, O, E = Error<I>> = Result<(I, O), nom::Err<E>>;

use crate::parser::schema::parse_schema;

use self::schema::Schema;

pub fn parse<'i>(input: impl Into<Span<'i>>) -> Result<Schema<'i>, Error<Span<'i>>> {
    final_parser(parse_schema)(input.into())
}
