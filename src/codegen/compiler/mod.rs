#![allow(unused_must_use)]

use std::{collections::HashMap, env, fs, path::Path};

use crate::schema::{Command, CustomType, Event, Schema};

use super::error::Error;

#[cfg(feature = "codegen-rust")]
pub mod rust;
#[cfg(feature = "codegen-typescript")]
pub mod typescript;

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
    #[cfg(feature = "wasm")]
    wasm: bool,
}

impl<C: Compile> Compiler<C> {
    /// Creates a new compiler instance.
    pub fn new(compiler: C) -> Self {
        Compiler {
            compiler,
            schemas: Vec::new(),
            #[cfg(feature = "wasm")]
            wasm: false,
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

    #[cfg(feature = "wasm")]
    pub fn wasm(mut self, enabled: bool) -> Self {
        self.wasm = enabled;
        self
    }

    /// Compile schemas into Rust code and save in OUT_DIR.
    pub fn compile(self) -> Result<(), Error> {
        let out_dir = env::var("OUT_DIR").unwrap();

        let compiler = &self.compiler;
        for schema in self.schemas {
            let code = compile_schema(
                compiler,
                &schema,
                #[cfg(feature = "wasm")]
                self.wasm,
            );
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
            let code = compile_schema(
                compiler,
                &schema,
                #[cfg(feature = "wasm")]
                self.wasm,
            );
            codes.insert(schema.aggregate.name, code);
        }

        Ok(codes)
    }
}

pub fn compile_schema<C: Compile>(
    compiler: &C,
    schema: &Schema,
    #[cfg(feature = "wasm")] wasm: bool,
) -> String {
    let mut code = String::new();

    compiler.compile_before(&mut code, schema);

    #[cfg(feature = "wasm")]
    if wasm {
        compiler.compile_wasm(&mut code, schema);
    }

    compiler.compile_schema_types(&mut code, &schema.types);

    compiler.compile_schema_events(&mut code, &schema.aggregate.name, &schema.events);

    compiler.compile_schema_commands(
        &mut code,
        &schema.aggregate.name,
        &schema.aggregate.commands,
    );

    compiler.compile_after(&mut code, schema);

    code
}

pub trait Compile {
    fn compile_before(&self, _code: &mut String, _schema: &Schema) {}
    fn compile_after(&self, _code: &mut String, _schema: &Schema) {}

    #[cfg(feature = "wasm")]
    fn compile_wasm(&self, _code: &mut String, _schema: &Schema) {}

    fn compile_schema_types(&self, code: &mut String, types: &HashMap<String, CustomType>);

    fn compile_schema_events(&self, code: &mut String, name: &str, events: &HashMap<String, Event>);

    fn compile_schema_commands(
        &self,
        code: &mut String,
        name: &str,
        commands: &HashMap<String, Command>,
    );
}
