use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Write},
};

use heck::ToUpperCamelCase;

use crate::schema::{
    Command, CommandEvents, CustomType, Event, EventOpt, RepeatableType, Scalar, Schema, TypeOpt,
    TypeRef,
};

use super::Compile;

#[cfg(feature = "codegen-rust-wasm")]
pub mod wasm;

/// Converts ESDL schema to Rust.
///
/// # Examples
///
/// **With build script**
///
/// To build Rust, add in a build script `build.rs`:
///
/// ```
/// use esdl::codegen::{rust::RustCompiler, Compiler};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     Compiler::new(RustCompiler)
///         .add_schema_file("./bank-account.esdl")?
///         .compile()?;
///
///     Ok(())
/// }
/// ```
///
/// And include in Rust code with the [include_aggregate](https://docs.rs/thalo/latest/thalo/macro.include_aggregate.html) macro.
///
/// ```
/// use thalo::include_aggregate;
///
/// include_aggregate!("BankAccount");
///
/// // Define aggregate...
/// ```
///
/// For a more complete example, see the docs at [include_aggregate](https://docs.rs/thalo/latest/thalo/macro.include_aggregate.html).
///
/// **WASM**
///
/// Wasm is supported through the "wasm" feature flag.
///
/// `esdl = { version = "*", features = ["codegen-rust-wasm"] }`
///
/// The following peer dependencies are required:
///
/// - `serde = { version = "1.0", features = ["derive"] }`
/// - `serde_json = "1.0"`
/// - `wit-bindgen-rust = { git = "https://github.com/bytecodealliance/wit-bindgen.git" }`
///
/// Additionally a `domain.wit` file should be in your workspace root:
///
/// ```text
/// // Errors that can occur
/// variant error {
///     command(list<u8>),
///     deserialize-state,
///     deserialize-event,
///     deserialize-command,
///     serialize-state,
///   }
///   
/// // Creates a new instance
/// // Returns state
/// new-instance: function(id: string) -> expected<list<u8>, error>
///
/// // Takes state, event
/// // Returns new state
/// apply-events: function(state: list<u8>, event: list<list<u8>>) -> expected<list<u8>, error>
///
/// // Takes state, command
/// // Returns list of events
/// handle-command: function(state: list<u8>, command: list<u8>) -> expected<list<list<u8>>, error>  
/// ```
///
/// Setup a build.rs file with wasm enabled:
///
/// ```
/// use esdl::codegen::{rust::RustCompiler, Compiler};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     Compiler::new(RustCompiler)
///         .add_schema_file("./bank-account.esdl")?
///         .wasm(true)
///         .compile()?;
///
///     Ok(())
/// }
/// ```
///
/// Finally, you should be setup to write your aggregate:
///
/// ```
/// use thalo::include_aggregate;
///
/// include_aggregate!("BankAccount");
///
/// // Define aggregate...
/// ```
///
/// It can be built with:
///
/// ```bash
/// $ cargo build --target wasm32-wasi --release
/// ```
#[derive(Default)]
pub struct RustCompiler;

impl Compile for RustCompiler {
    fn compile_schema(&self, schema: &Schema) -> String {
        let mut code = String::new();

        self.compile_schema_types(&mut code, &schema.types);

        self.compile_schema_events(&mut code, &schema.aggregate.name, &schema.events);

        self.compile_schema_commands(
            &mut code,
            &schema.aggregate.name,
            &schema.aggregate.commands,
        );

        code
    }
}

impl RustCompiler {
    fn compile_schema_types(&self, code: &mut String, types: &HashMap<String, CustomType>) {
        for (type_name, ty) in types {
            let derives = ty
                .fields
                .derives()
                .into_iter()
                .map(|derive| derive.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(code, "#[derive({derives})]");
            writeln!(code, "pub struct {} {{", type_name);
            for (field_name, field) in &ty.fields {
                writeln!(code, "    pub {}: {},", field_name, field.to_rust_type());
            }
            writeln!(code, "}}\n");
        }
    }

    fn compile_schema_events(
        &self,
        code: &mut String,
        name: &str,
        events: &HashMap<String, Event>,
    ) {
        writeln!(
            code,
            "#[derive(Clone, Debug, serde::Deserialize, thalo::event::EventType, PartialEq, serde::Serialize)]"
        );
        writeln!(code, r##"#[serde(tag = "event", content = "data")]"##);
        writeln!(code, "pub enum {}Event {{", name);

        for event_name in events.keys() {
            writeln!(code, "    {}({0}Event),", event_name);
        }

        writeln!(code, "}}\n");

        for (event_name, event) in events {
            let derives = event
                .fields
                .derives()
                .into_iter()
                .map(|derive| derive.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(code, "#[derive({derives}, thalo::event::Event)]");
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

    fn compile_schema_commands(
        &self,
        code: &mut String,
        name: &str,
        commands: &HashMap<String, Command>,
    ) {
        writeln!(
            code,
            "#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]"
        );
        writeln!(code, r##"#[serde(tag = "command", content = "params")]"##);
        writeln!(code, "pub enum {}CommandEnum {{", name);
        for (command_name, command) in commands {
            let command_name_camel_case = command_name.to_upper_camel_case();
            writeln!(code, r##"    #[serde(rename = "{}")]"##, command_name);
            writeln!(code, "    {} {{", command_name_camel_case);
            for param in &command.params {
                writeln!(code, "        {}: {},", param.name, param.ty.to_rust_type());
            }
            writeln!(code, "    }},");
        }
        writeln!(code, "}}");
        writeln!(code);
        writeln!(code, "pub trait {}Command {{", name);
        writeln!(code, "    type Error;");
        writeln!(code);
        for (command_name, command) in commands {
            write!(code, "    fn {}(&self", command_name);

            for param in &command.params {
                write!(code, ", {}: {}", param.name, param.ty.to_rust_type());
            }

            writeln!(
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

impl ToRustType for CommandEvents {
    fn to_rust_type(&self) -> String {
        match self {
            CommandEvents::Single(event_opt) => event_opt.to_rust_type(),
            CommandEvents::Tuple(events) => format!(
                "({})",
                events
                    .iter()
                    .map(|event| event.to_rust_type())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl ToRustType for EventOpt {
    fn to_rust_type(&self) -> String {
        match self {
            EventOpt::Optional(event) => format!("std::option::Option<{}Event>", event.name),
            EventOpt::Required(event) => format!("{}Event", event.name),
        }
    }
}

impl ToRustType for RepeatableType {
    fn to_rust_type(&self) -> String {
        match self {
            RepeatableType::Single(type_opt) => type_opt.to_rust_type(),
            RepeatableType::OptionalArray(type_opt) => format!(
                "std::option::Option<std::vec::Vec<{}>>",
                type_opt.to_rust_type()
            ),
            RepeatableType::RequiredArray(type_opt) => {
                format!("std::vec::Vec<{}>", type_opt.to_rust_type())
            }
        }
    }
}

impl ToRustType for TypeOpt {
    fn to_rust_type(&self) -> String {
        match self {
            TypeOpt::Optional(type_ref) => {
                format!("std::option::Option<{}>", type_ref.to_rust_type())
            }
            TypeOpt::Required(type_ref) => type_ref.to_rust_type(),
        }
    }
}

impl ToRustType for TypeRef {
    fn to_rust_type(&self) -> String {
        match self {
            TypeRef::Scalar(scalar) => scalar.to_rust_type(),
            TypeRef::Custom(custom_type) => custom_type.name.clone(),
        }
    }
}

impl ToRustType for Scalar {
    fn to_rust_type(&self) -> String {
        match self {
            Scalar::String => "String".to_string(),
            Scalar::Int => "i64".to_string(),
            Scalar::Float => "f64".to_string(),
            Scalar::Bool => "bool".to_string(),
            Scalar::Timestamp => "chrono::DateTime<chrono::FixedOffset>".to_string(),
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum DeriveTrait {
    Clone,
    Copy,
    Hash,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
}

impl DeriveTrait {
    fn all() -> HashSet<Self> {
        HashSet::from_iter([
            DeriveTrait::Clone,
            DeriveTrait::Copy,
            DeriveTrait::Hash,
            DeriveTrait::Debug,
            DeriveTrait::Default,
            DeriveTrait::Eq,
            DeriveTrait::PartialEq,
            DeriveTrait::Ord,
            DeriveTrait::PartialOrd,
            DeriveTrait::Serialize,
            DeriveTrait::Deserialize,
        ])
    }
}

impl fmt::Display for DeriveTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DeriveTrait::*;

        match self {
            Clone => write!(f, "Clone"),
            Copy => write!(f, "Copy"),
            Hash => write!(f, "Hash"),
            Debug => write!(f, "Debug"),
            Default => write!(f, "Default"),
            Eq => write!(f, "Eq"),
            PartialEq => write!(f, "PartialEq"),
            Ord => write!(f, "Ord"),
            PartialOrd => write!(f, "PartialOrd"),
            Serialize => write!(f, "serde::Serialize"),
            Deserialize => write!(f, "serde::Deserialize"),
        }
    }
}

trait RustTypeDerives {
    fn derives(&self) -> HashSet<DeriveTrait>;
}

impl RustTypeDerives for HashMap<String, RepeatableType> {
    fn derives(&self) -> HashSet<DeriveTrait> {
        self.iter()
            .fold(DeriveTrait::all(), |acc, (_, ty)| &acc & &ty.derives())
    }
}

impl RustTypeDerives for RepeatableType {
    fn derives(&self) -> HashSet<DeriveTrait> {
        match self {
            RepeatableType::Single(type_opt) => type_opt.derives(),
            RepeatableType::OptionalArray(type_opt) => type_opt.derives(),
            RepeatableType::RequiredArray(type_opt) => type_opt.derives(),
        }
    }
}

impl RustTypeDerives for TypeOpt {
    fn derives(&self) -> HashSet<DeriveTrait> {
        match self {
            TypeOpt::Optional(type_ref) => type_ref.derives(),
            TypeOpt::Required(type_ref) => type_ref.derives(),
        }
    }
}

impl RustTypeDerives for TypeRef {
    fn derives(&self) -> HashSet<DeriveTrait> {
        match self {
            TypeRef::Scalar(scalar) => scalar.derives(),
            TypeRef::Custom(custom_type) => custom_type.fields.derives(),
        }
    }
}

impl RustTypeDerives for Scalar {
    fn derives(&self) -> HashSet<DeriveTrait> {
        use DeriveTrait::*;

        match self {
            Scalar::String => HashSet::from_iter([
                Clone,
                Hash,
                Debug,
                Default,
                Eq,
                PartialEq,
                Ord,
                PartialOrd,
                Serialize,
                Deserialize,
            ]),
            Scalar::Int => HashSet::from_iter([
                Clone,
                Copy,
                Hash,
                Debug,
                Default,
                Eq,
                PartialEq,
                Ord,
                PartialOrd,
                Serialize,
                Deserialize,
            ]),
            Scalar::Float => HashSet::from_iter([
                Clone,
                Copy,
                Debug,
                Default,
                PartialEq,
                PartialOrd,
                Serialize,
                Deserialize,
            ]),
            Scalar::Bool => DeriveTrait::all(),
            Scalar::Timestamp => DeriveTrait::all(),
        }
    }
}
