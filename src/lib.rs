#![doc = include_str!("../README.md")]

#[cfg(feature = "codegen")]
pub use codegen::configure;
pub use parser::parse;

#[cfg(feature = "codegen")]
mod codegen;
pub(crate) mod parser;
pub mod schema;
