//! Generate code from ESDL schemas.
//!
//! Supported languages:
//!
//!   - [Rust](rust::RustCompiler)
//!   - [TypeScript](typescript::TypeScriptCompiler)

pub use compiler::*;
pub use error::Error;

mod compiler;
mod error;
