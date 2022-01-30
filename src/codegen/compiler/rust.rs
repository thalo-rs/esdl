use std::{collections::HashMap, fmt::Write};

use heck::ToUpperCamelCase;

#[cfg(feature = "wasm")]
use crate::schema::Schema;
use crate::schema::{
    Command, CommandEvents, CustomType, Event, EventOpt, RepeatableType, Scalar, TypeOpt, TypeRef,
};

use super::Compile;

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
/// `esdl = { version = "*", features = ["codegen-rust", "wasm"] }`
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
    #[cfg(feature = "wasm")]
    fn compile_wasm(&self, code: &mut String, schema: &Schema) {
        writeln!(code, r#"wit_bindgen_rust::export!("./domain.wit");"#);
        writeln!(code);
        writeln!(code, "struct Domain;");
        writeln!(code);
        writeln!(code, "impl domain::Domain for Domain {{");
        // New instance
        writeln!(
            code,
            "    fn new_instance(id: String) -> Result<Vec<u8>, domain::Error> {{"
        );
        writeln!(
            code,
            "        let state = {}::new(id);",
            schema.aggregate.name
        );
        writeln!(code);
        writeln!(
            code,
            "        serde_json::to_vec(&state).map_err(|_| domain::Error::SerializeState)"
        );
        writeln!(code, "    }}");
        writeln!(code);

        // Apply events
        writeln!(code, "    fn apply_events(state: Vec<u8>, events: Vec<Vec<u8>>) -> Result<Vec<u8>, domain::Error> {{");
        writeln!(code, "        let mut state: {} =", schema.aggregate.name);
        writeln!(code, "            serde_json::from_slice(&state).map_err(|_| domain::Error::DeserializeState)?;");
        writeln!(
            code,
            "        let events: Vec<{}Event> = events",
            schema.aggregate.name
        );
        writeln!(code, "            .into_iter()");
        writeln!(code, "            .map(|event| {{");
        writeln!(code, "                serde_json::from_slice(&event).map_err(|_| domain::Error::DeserializeEvent)");
        writeln!(code, "            }})");
        writeln!(code, "            .collect::<Result<_, _>>()?;");
        writeln!(code);
        writeln!(code, "        for event in events {{");
        writeln!(code, "            state.apply(event);");
        writeln!(code, "        }}");
        writeln!(code);
        writeln!(
            code,
            "        serde_json::to_vec(&state).map_err(|_| domain::Error::SerializeState)"
        );
        writeln!(code, "    }}");
        writeln!(code);

        // Handle command
        writeln!(code, "    fn handle_command(state: Vec<u8>, command: Vec<u8>) -> Result<Vec<Vec<u8>>, domain::Error> {{");
        writeln!(code, "        use thalo::event::IntoEvents;");
        writeln!(code);
        writeln!(code, "        let state: BankAccount =");
        writeln!(code, "            serde_json::from_slice(&state).map_err(|_| domain::Error::DeserializeState)?;");
        writeln!(
            code,
            "        let command: {}CommandEnum =",
            schema.aggregate.name
        );
        writeln!(code, "            serde_json::from_slice(&command).map_err(|_| domain::Error::DeserializeCommand)?;");
        writeln!(code);
        writeln!(code, "        let events = match command {{");
        for (command_name, command) in &schema.aggregate.commands {
            let command_name_camel_case = command_name.to_upper_camel_case();
            writeln!(
                code,
                "            {}CommandEnum::{} {{",
                schema.aggregate.name, command_name_camel_case
            );
            for param in &command.params {
                writeln!(code, "                {},", param.name);
            }
            writeln!(code, "            }} => state");
            write!(code, "                .{}(", command_name);
            let params_len = command.params.len();
            for (i, param) in command.params.iter().enumerate() {
                write!(code, "{}", param.name);
                if i < params_len - 1 {
                    write!(code, ", ");
                }
            }
            writeln!(code, ")");
            writeln!(code, "                .map_err(|err| {{");
            writeln!(
                code,
                "                    domain::Error::Command(err.to_string().into_bytes())"
            );
            writeln!(code, "                }})?");
            writeln!(code, "                .into_events(),");
        }
        writeln!(code, "        }};");
        writeln!(code);
        writeln!(code, "        Ok(events");
        writeln!(code, "            .into_iter()");
        writeln!(
            code,
            "            .filter_map(|event| serde_json::to_vec(&event).ok())"
        );
        writeln!(code, "            .collect())");
        writeln!(code, "    }}");
        writeln!(code, "}}");
        writeln!(code);
    }

    fn compile_schema_types(&self, code: &mut String, types: &HashMap<String, CustomType>) {
        for (type_name, ty) in types {
            writeln!(
                code,
                "#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]"
            );
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
        #[cfg(feature = "wasm")]
        writeln!(code, "    type Error: std::fmt::Display;");
        #[cfg(not(feature = "wasm"))]
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
