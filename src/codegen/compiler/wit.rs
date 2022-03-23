use std::{collections::HashMap, fmt::Write};

use heck::ToKebabCase;

use crate::schema::{
    Command, CommandEvents, CustomType, Event, EventOpt, RepeatableType, Scalar, Schema, TypeOpt,
    TypeRef,
};

use super::Compile;

/// Converts ESDL schema to Wit files for WASI.
///
/// # Example
///
/// To build TypeScript, in `main.rs`:
///
/// ```
/// use esdl::codegen::{wit::WitCompiler, Compiler};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let schemas = Compiler::new(WitCompiler)
///         .add_schema_file("./bank-account.esdl")?
///         .compile_to_strings()?;
///
///     for (aggregate_name, code) in schemas {
///         std::fs::write(format!("./{}.wit", aggregate_name), code)?;
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Default)]
pub struct WitCompiler;

impl Compile for WitCompiler {
    fn compile_before(&self, code: &mut String, _schema: &Schema) {
        writeln!(
            code,
            "// Generated using ESDL compiler <https://github.com/thalo-rs/esdl>"
        );
        writeln!(code);
    }

    fn compile_schema_types(&self, code: &mut String, types: &HashMap<String, CustomType>) {
        for (type_name, ty) in types {
            writeln!(code, "record {} {{", type_name.to_kebab_case());
            for (field_name, field) in &ty.fields {
                writeln!(
                    code,
                    "    {}: {},",
                    field_name.to_kebab_case(),
                    field.to_wit_type()
                );
            }
            writeln!(code, "}};\n");
        }
    }

    fn compile_schema_events(
        &self,
        code: &mut String,
        _name: &str,
        events: &HashMap<String, Event>,
    ) {
        // writeln!(code, "variant {}-event {{", name.to_kebab_case());

        // for event_name in events.keys() {
        //     writeln!(
        //         code,
        //         "    {name}({name}-event),",
        //         name = event_name.to_kebab_case()
        //     );
        // }

        // writeln!(code, "}}\n");

        for (event_name, event) in events {
            writeln!(code, "record {}-event {{", event_name.to_kebab_case());
            for (field_name, field) in &event.fields {
                writeln!(
                    code,
                    "    {}: {},",
                    field_name.to_kebab_case(),
                    field.to_wit_type()
                );
            }
            writeln!(code, "}}\n");
        }
    }

    fn compile_schema_commands(
        &self,
        code: &mut String,
        _name: &str,
        commands: &HashMap<String, Command>,
    ) {
        for (command_name, command) in commands {
            write!(
                code,
                "{}: function(state: list<u8>",
                command_name.to_kebab_case()
            );

            for param in &command.params {
                write!(
                    code,
                    ", {}: {}",
                    param.name.to_kebab_case(),
                    param.ty.to_wit_type()
                );
            }

            writeln!(
                code,
                ") -> expected<{}, list<u8>>",
                command.events.to_wit_type()
            );
            writeln!(code);
        }
    }
}

pub trait ToWitType {
    fn to_wit_type(&self) -> String;
}

impl ToWitType for CommandEvents {
    fn to_wit_type(&self) -> String {
        match self {
            CommandEvents::Single(event_opt) => event_opt.to_wit_type(),
            CommandEvents::Tuple(events) => format!(
                "tuple<{}>",
                events
                    .iter()
                    .map(|event| event.to_wit_type())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl ToWitType for EventOpt {
    fn to_wit_type(&self) -> String {
        match self {
            EventOpt::Optional(event) => format!("option<{}-event>", event.name.to_kebab_case()),
            EventOpt::Required(event) => format!("{}-event", event.name.to_kebab_case()),
        }
    }
}

impl ToWitType for RepeatableType {
    fn to_wit_type(&self) -> String {
        match self {
            RepeatableType::Single(type_opt) => type_opt.to_wit_type(),
            RepeatableType::OptionalArray(type_opt) => {
                format!("option<list<{}>>", type_opt.to_wit_type())
            }
            RepeatableType::RequiredArray(type_opt) => {
                format!("list<{}>", type_opt.to_wit_type())
            }
        }
    }
}

impl ToWitType for TypeOpt {
    fn to_wit_type(&self) -> String {
        match self {
            TypeOpt::Optional(type_ref) => {
                format!("option<{}>", type_ref.to_wit_type())
            }
            TypeOpt::Required(type_ref) => type_ref.to_wit_type(),
        }
    }
}

impl ToWitType for TypeRef {
    fn to_wit_type(&self) -> String {
        match self {
            TypeRef::Scalar(scalar) => scalar.to_wit_type(),
            TypeRef::Custom(custom_type) => custom_type.name.to_kebab_case(),
        }
    }
}

impl ToWitType for Scalar {
    fn to_wit_type(&self) -> String {
        match self {
            Scalar::String => "string".to_string(),
            Scalar::Int => "s64".to_string(),
            Scalar::Float => "f64".to_string(),
            Scalar::Bool => "bool".to_string(),
            Scalar::Timestamp => "string".to_string(),
        }
    }
}
