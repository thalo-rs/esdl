#![doc = include_str!("../README.md")]

pub use parser::parse;

#[cfg(feature = "codegen")]
pub use codegen::rust::configure;

#[cfg(feature = "codegen")]
pub mod codegen;
pub(crate) mod parser;
pub mod schema;
