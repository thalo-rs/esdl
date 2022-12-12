#![doc = include_str!("../README.md")]
//!
//! ## Code generation
//!
//! For usage with code generation, see [codegen].

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod error;
pub(crate) mod parser;
pub mod schema;

pub use error::Error;

/// Parse and validate an ESDL schema string.
pub fn parse(input: &str) -> Result<schema::Schema, Error> {
    let schema = parser::parse(input)?;
    schema::Schema::validate_parsed_schema(schema)
}
