#![doc = include_str!("../README.md")]

//!
//! ## Code generation
//!
//! For usage with code generation, see [codegen].

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub use parser::parse;

#[cfg(any(feature = "codegen-rust", feature = "codegen-typescript"))]
pub mod codegen;
pub(crate) mod parser;
pub mod schema;
