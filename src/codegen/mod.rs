#![allow(unused_must_use)]

use std::{collections::HashMap, env, fmt::Write, fs, path::Path};

use crate::schema::{Command, CustomType, Event, Schema};

use self::error::Error;

mod error;

/// Configure a compiler.
pub fn configure() -> Compiler {
    Compiler::new()
}

/// Compile schemas into Rust code.
///
/// # Example
///
/// ```
/// // build.rs
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     thalo_schema::configure()
///         .add_schema_file("./bank-account.agg")?
///         .compile()?;
///
///     Ok(())
/// }
/// ```
#[derive(Default)]
pub struct Compiler {
    schemas: Vec<Schema>,
}

impl Compiler {
    /// Creates a new compiler instance.
    pub fn new() -> Self {
        Compiler {
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

    /// Compile schemas into Rust code and save in OUT_DIR.
    pub fn compile(self) -> Result<(), Error> {
        let out_dir = env::var("OUT_DIR").unwrap();

        for schema in self.schemas {
            let code = Self::compile_schema(&schema);
            fs::write(format!("{}/{}.rs", out_dir, schema.aggregate.name), code)?;
        }

        Ok(())
    }

    /// Compile schemas into Rust code and outputs as String.
    pub fn compile_to_string(self) -> Result<String, Error> {
        let mut codes = Vec::new();

        for schema in self.schemas {
            let code = Self::compile_schema(&schema);
            codes.push(code);
        }

        Ok(codes.join("\n\n"))
    }

    fn compile_schema(schema: &Schema) -> String {
        let mut code = String::new();

        Self::compile_schema_types(&mut code, &schema.types);

        Self::compile_schema_events(&mut code, &schema.aggregate.name, &schema.events);

        Self::compile_schema_commands(
            &mut code,
            &schema.aggregate.name,
            &schema.aggregate.commands,
        );

        code
    }

    fn compile_schema_types(code: &mut String, types: &HashMap<String, CustomType>) {
        for (type_name, ty) in types {
            writeln!(
                code,
                "#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]"
            );
            writeln!(code, "pub struct {} {{", type_name);
            for (field_name, field) in &ty.fields {
                writeln!(code, "    pub {}: {},", field_name, field.to_rust_type());
            }
            writeln!(code, "}}\n");
        }
    }

    fn compile_schema_events(code: &mut String, name: &str, events: &HashMap<String, Event>) {
        writeln!(
            code,
            "#[derive(Clone, Debug, serde::Deserialize, thalo::event::EventType, PartialEq, serde::Serialize)]"
        );
        writeln!(code, "pub enum {}Event {{", name);

        for event_name in events.keys() {
            writeln!(code, "    {}({0}Event),", event_name);
        }

        writeln!(code, "}}\n");

        for (event_name, event) in events {
            writeln!(code, "#[derive(Clone, Debug, serde::Deserialize, thalo::event::Event, PartialEq, serde::Serialize)]");
            writeln!(
                code,
                r#"#[thalo(parent = "{}Event", variant = "{}")]"#,
                name, event_name
            );
            writeln!(code, "pub struct {}Event {{", event_name);
            for (field_name, field) in &event.fields {
                writeln!(code, "    pub {}: {},", field_name, field.to_rust_type());
            }
            writeln!(code, "}}\n");
        }
    }

    fn compile_schema_commands(code: &mut String, name: &str, commands: &HashMap<String, Command>) {
        writeln!(code, "pub trait {}Command {{", name);
        writeln!(code, "    type Error;");
        writeln!(code);

        for (command_name, command) in commands {
            write!(code, "    fn {}(&self", command_name);

            for param in &command.params {
                write!(code, ", {}: {}", param.name, param.ty.to_rust_type());
            }

            write!(
                code,
                ") -> std::result::Result<{}, Self::Error>;",
                command.events.to_rust_type()
            );
        }

        writeln!(code, "}}");
    }
}

pub trait ToRustType {
    fn to_rust_type(&self) -> String;
}
