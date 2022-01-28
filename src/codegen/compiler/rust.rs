use std::{collections::HashMap, fmt::Write};

use crate::schema::{
    Command, CommandEvents, CustomType, Event, EventOpt, RepeatableType, Scalar, TypeOpt, TypeRef,
};

use super::Compile;

/// Converts ESDL schema to Rust.
///
/// # Example
///
/// To build Rust, add in a build script `build.rs`:
///
/// ```
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     esdl::configure()
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
#[derive(Default)]
pub struct RustCompiler;

impl Compile for RustCompiler {
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
