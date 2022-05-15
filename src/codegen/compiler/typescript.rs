use std::{collections::HashMap, fmt::Write};

use crate::schema::{
    Command, CommandEvents, CustomType, Event, EventOpt, RepeatableType, Scalar, Schema, TypeOpt,
    TypeRef,
};

use super::Compile;

/// Converts ESDL schema to TypeScript.
///
/// # Example
///
/// To build TypeScript, in `main.rs`:
///
/// ```
/// use esdl::codegen::{typescript::TypeScriptCompiler, Compiler};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let schemas = Compiler::new(TypeScriptCompiler)
///         .add_schema_file("./bank-account.esdl")?
///         .compile_to_strings()?;
///
///     for (aggregate_name, code) in schemas {
///         std::fs::write(format!("./{}.ts", aggregate_name), code)?;
///     }
///
///     Ok(())
/// }
/// ```
///
/// The resulting TypeScript will be similar to:
///
/// ```typescript
/// import type { Event } from 'thalo';
///
/// export type BankAccountEvent =
///   | WithdrewFundsEvent
///   | OpenedAccountEvent
///   | DepositedFundsEvent;
///
/// export type WithdrewFundsEvent = Event<
///   'WithdrewFunds',
///   {
///     amount: number;
///   }
/// >;
///
/// export type OpenedAccountEvent = Event<
///   'OpenedAccount',
///   {
///     initial_balance: number;
///   }
/// >;
///
/// export type DepositedFundsEvent = Event<
///   'DepositedFunds',
///   {
///     amount: number;
///   }
/// >;
///
/// export interface BankAccountCommand {
///   deposit_funds(amount: number): DepositedFundsEvent;
///   withdraw_funds(amount: number): WithdrewFundsEvent;
///   open_account(initial_balance: number): OpenedAccountEvent;
/// }
/// ```
#[derive(Default)]
pub struct TypeScriptCompiler;

impl Compile for TypeScriptCompiler {
    fn compile_schema(&self, schema: &Schema) -> String {
        let mut code = String::new();

        self.compile_before(&mut code);
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

impl TypeScriptCompiler {
    fn compile_before(&self, code: &mut String) {
        writeln!(code, "import type {{ Event }} from 'thalo';");
        writeln!(code);
    }

    fn compile_schema_types(&self, code: &mut String, types: &HashMap<String, CustomType>) {
        for (type_name, ty) in types {
            writeln!(code, "export type {} = {{", type_name);
            for (field_name, field) in &ty.fields {
                writeln!(code, "  {}: {},", field_name, field.to_ts_type());
            }
            writeln!(code, "}};\n");
        }
    }

    fn compile_schema_events(
        &self,
        code: &mut String,
        name: &str,
        events: &HashMap<String, Event>,
    ) {
        writeln!(code, "export type {}Event =", name);

        let event_keys_len = events.keys().len();
        for (i, event_name) in events.keys().enumerate() {
            write!(code, "  | {}Event", event_name);
            if i == event_keys_len - 1 {
                writeln!(code, ";");
            } else {
                writeln!(code);
            }
        }

        writeln!(code);

        for (event_name, event) in events {
            writeln!(code, "export type {}Event = Event<", event_name);
            writeln!(code, "  '{}',", event_name);
            writeln!(code, "  {{");
            for (field_name, field) in &event.fields {
                writeln!(code, "    {}: {};", field_name, field.to_ts_type());
            }
            writeln!(code, "  }}");
            writeln!(code, ">;\n");
        }
    }

    fn compile_schema_commands(
        &self,
        code: &mut String,
        name: &str,
        commands: &HashMap<String, Command>,
    ) {
        writeln!(code, "export interface {}Command {{", name);

        for (command_name, command) in commands {
            write!(code, "  {}(", command_name);

            for (i, param) in command.params.iter().enumerate() {
                if i > 0 {
                    write!(code, ", ");
                }
                write!(code, "{}: {}", param.name, param.ty.to_ts_type());
            }

            writeln!(code, "): {};", command.events.to_ts_type());
        }

        writeln!(code, "}}");
    }
}

pub trait ToTsType {
    fn to_ts_type(&self) -> String;
}

impl ToTsType for CommandEvents {
    fn to_ts_type(&self) -> String {
        match self {
            CommandEvents::Single(event_opt) => event_opt.to_ts_type(),
            CommandEvents::Tuple(events) => format!(
                "[{}]",
                events
                    .iter()
                    .map(|event| event.to_ts_type())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl ToTsType for EventOpt {
    fn to_ts_type(&self) -> String {
        match self {
            EventOpt::Optional(event) => format!("{} | undefined", event.name),
            EventOpt::Required(event) => format!("{}Event", event.name),
        }
    }
}

impl ToTsType for RepeatableType {
    fn to_ts_type(&self) -> String {
        match self {
            RepeatableType::Single(type_opt) => type_opt.to_ts_type(),
            RepeatableType::OptionalArray(type_opt) => {
                format!("{}[] | undefined", type_opt.to_ts_type())
            }
            RepeatableType::RequiredArray(type_opt) => {
                format!("{}[]", type_opt.to_ts_type())
            }
        }
    }
}

impl ToTsType for TypeOpt {
    fn to_ts_type(&self) -> String {
        match self {
            TypeOpt::Optional(type_ref) => {
                format!("{} | undefined", type_ref.to_ts_type())
            }
            TypeOpt::Required(type_ref) => type_ref.to_ts_type(),
        }
    }
}

impl ToTsType for TypeRef {
    fn to_ts_type(&self) -> String {
        match self {
            TypeRef::Scalar(scalar) => scalar.to_ts_type(),
            TypeRef::Custom(custom_type) => custom_type.name.clone(),
        }
    }
}

impl ToTsType for Scalar {
    fn to_ts_type(&self) -> String {
        match self {
            Scalar::String => "string".to_string(),
            Scalar::Int => "number".to_string(),
            Scalar::Float => "number".to_string(),
            Scalar::Bool => "boolean".to_string(),
            Scalar::Timestamp => "Date".to_string(),
        }
    }
}
