#![allow(unused_must_use)]

use std::{collections::HashMap, env, fs, path::Path};

use crate::schema::Schema;

use super::error::Error;

#[cfg(any(feature = "codegen-rust", feature = "codegen-rust-wasm"))]
pub mod rust;
#[cfg(feature = "codegen-typescript")]
pub mod typescript;
#[cfg(feature = "codegen-wit")]
pub mod wit;

/// Compile schemas into Rust code.
///
/// # Example
///
/// ```
/// use esdl::codegen::{rust::RustCompiler, Compiler};
///
/// Compiler::new(RustCompiler)
///     .add_schema_file("./bank-account.esdl")?
///     .compile()?;
/// ```
#[derive(Default)]
pub struct Compiler<C> {
    compiler: C,
    schemas: Vec<Schema>,
}

impl<C: Compile> Compiler<C> {
    /// Creates a new compiler instance.
    pub fn new(compiler: C) -> Self {
        Compiler {
            compiler,
            schemas: Vec::new(),
        }
    }

    /// Adds a schema.
    pub fn add_schema(mut self, schema: Schema) -> Self {
        self.schemas.push(schema);
        self
    }

    /// Add a schema from a yaml file.
    pub fn add_schema_file<P: AsRef<Path>>(self, path: P) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        self.add_schema_str(&content)
    }

    /// Add a schema from yaml string.
    pub fn add_schema_str(self, content: &str) -> Result<Self, Error> {
        let parsed_schema = crate::parse(content).map_err(|err| Error::Parse(err.to_string()))?;
        let schema = Schema::validate_parsed_schema(parsed_schema)?;
        Ok(self.add_schema(schema))
    }

    /// Compiles wasm code.
    pub fn with_wasm(self) -> Compiler<<C as IntoWasmCompiler>::Compiler>
    where
        C: IntoWasmCompiler,
    {
        Compiler {
            compiler: self.compiler.into_wasm_compiler(),
            schemas: self.schemas,
        }
    }

    /// Compile schemas into Rust code and save in OUT_DIR.
    pub fn compile(self) -> Result<(), Error> {
        let out_dir = env::var("OUT_DIR").unwrap();

        let compiler = &self.compiler;
        for schema in self.schemas {
            let code = compiler.compile_schema(&schema);
            fs::write(format!("{}/{}.rs", out_dir, schema.aggregate.name), code)?;
        }

        Ok(())
    }

    /// Compile schemas into Rust code and outputs as hashmap.
    ///
    /// The resulting hashmap key is the aggregate name, and the value is the generated code.
    pub fn compile_to_strings(self) -> Result<HashMap<String, String>, Error> {
        let mut codes = HashMap::with_capacity(self.schemas.len());

        let compiler = &self.compiler;
        for schema in self.schemas {
            let code = compiler.compile_schema(&schema);
            codes.insert(schema.aggregate.name, code);
        }

        Ok(codes)
    }
}

pub trait Compile {
    fn compile_schema(&self, schema: &Schema) -> String;
}

pub trait IntoWasmCompiler {
    type Compiler: Compile;

    fn into_wasm_compiler(self) -> Self::Compiler;
}
