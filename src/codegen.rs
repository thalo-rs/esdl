//! Generate code from ESDL schemas.
//!
//! Supported languages:
//!
//!   - [Rust](rust::RustCompiler) (`codegen-rust`)
//!   - [Rust WASM](rust::wasm::RustWasmCompiler) (`codegen-rust-wasm`)
//!   - [TypeScript](typescript::TypeScriptCompiler) (`codegen-typescript`)

pub use compiler::*;

mod compiler;
