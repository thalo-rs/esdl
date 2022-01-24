use std::collections::HashMap;

pub use error::Error;

#[cfg(feature = "codegen")]
use crate::codegen::ToRustType;

mod error;

/// Schema definition including aggregate, commands, events & custom types.
#[derive(Clone, Debug, PartialEq)]
pub struct Schema {
    pub aggregate: Aggregate,
    pub events: HashMap<String, Event>,
    pub types: HashMap<String, CustomType>,
}

impl Schema {
    pub fn validate_parsed_schema(schema: crate::parser::schema::Schema) -> Result<Self, Error> {
        let schema_types: HashMap<_, _> = schema
            .types
            .iter()
            .map(|ty| (ty.ident, &ty.fields))
            .collect();

        let types = schema
            .types
            .iter()
            .try_fold(HashMap::new(), |mut acc, ty| {
                let name = ty.ident.to_string();
                let custom_type = CustomType::from_custom_type(&schema_types, ty)?;
                if acc.insert(name.clone(), custom_type).is_some() {
                    return Err(Error::DuplicateCustomType(name));
                }

                Ok(acc)
            })?;

        let events = schema
            .events
            .iter()
            .try_fold(HashMap::new(), |mut acc, event| {
                let name = event.ident.to_string();
                let event = Event::from_event(&schema_types, event)?;
                if acc.insert(name.clone(), event).is_some() {
                    return Err(Error::DuplicateEvent(name));
                }

                Ok(acc)
            })?;

        let aggregate = if schema.aggregates.len() > 1 {
            return Err(Error::MultipleAggregates);
        } else {
            match schema.aggregates.first() {
                Some(aggregate) => Aggregate::from_aggregate(aggregate, &schema_types, &events)?,
                None => return Err(Error::MissingAggregate),
            }
        };

        Ok(Schema {
            aggregate,
            events,
            types,
        })
    }
}

/// Aggregate definition with name and commands.
///
/// ```text
/// aggregate BankAccount  {
///   open_account(user: User!, initial_balance: Float): OpenedAccount
///   make_transaction(amount: Float): (DepositedFunds! | WithdrewFunds!)
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Aggregate {
    pub name: String,
    pub commands: HashMap<String, Command>,
}

impl Aggregate {
    fn from_aggregate(
        aggregate: &crate::parser::aggregate::Aggregate,
        custom_types: &HashMap<&str, &Vec<crate::parser::event::Field>>,
        events: &HashMap<String, Event>,
    ) -> Result<Self, Error> {
        let name = aggregate.ident.to_string();
        let commands = aggregate
            .commands
            .iter()
            .try_fold(HashMap::new(), |mut acc, command| {
                if acc
                    .insert(
                        command.ident.to_string(),
                        Command::from_command(command, custom_types, events)?,
                    )
                    .is_some()
                {
                    return Err(Error::DuplicateCommand(command.ident.to_string()));
                }

                Ok(acc)
            })?;

        Ok(Aggregate { name, commands })
    }
}

/// Command definition with name, params and resulting events.
/// - `open_account(initial_balance: Float!): OpenedAccount`
/// - `make_transaction(amount: Float!): (DepositedFunds | WithdrewFunds!)`
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub params: Vec<Param>,
    pub events: CommandEvents,
}

impl Command {
    fn from_command(
        command: &crate::parser::aggregate::Command,
        custom_types: &HashMap<&str, &Vec<crate::parser::event::Field>>,
        events: &HashMap<String, Event>,
    ) -> Result<Self, Error> {
        let name = command.ident.to_string();
        let params = command
            .params
            .iter()
            .try_fold(Vec::<Param>::new(), |mut acc, param| {
                let param_name = param.ident.to_string();
                if acc.iter().any(|param| param.name == param_name) {
                    return Err(Error::DuplicateParam {
                        command: name.clone(),
                        param: param_name,
                    });
                }

                acc.push(Param::from_param(custom_types, param)?);

                Ok(acc)
            })?;
        let events = CommandEvents::from_return_type(&command.return_type, events)?;

        Ok(Command {
            name,
            params,
            events,
        })
    }
}

/// Command parameter with name and type.
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: RepeatableType,
}

impl Param {
    fn from_param(
        custom_types: &HashMap<&str, &Vec<crate::parser::event::Field>>,
        param: &crate::parser::aggregate::Param,
    ) -> Result<Self, Error> {
        let name = param.ident.to_string();
        let ty = RepeatableType::from_type(custom_types, &param.ty)?;

        Ok(Param { name, ty })
    }
}

/// Events resulted by a command.
/// - `Event`
/// - `(EventOne, EventTwo!)`
#[derive(Clone, Debug, PartialEq)]
pub enum CommandEvents {
    Single(EventOpt),
    Tuple(Vec<EventOpt>),
}

impl CommandEvents {
    fn from_return_type(
        return_type: &crate::parser::aggregate::ReturnType,
        events: &HashMap<String, Event>,
    ) -> Result<Self, Error> {
        match return_type {
            crate::parser::aggregate::ReturnType::Single(return_type_optional_or_required) => Ok(
                CommandEvents::Single(EventOpt::from_return_type_optional_or_required(
                    return_type_optional_or_required,
                    events,
                )?),
            ),
            crate::parser::aggregate::ReturnType::Tuple(return_events) => Ok(CommandEvents::Tuple(
                return_events
                    .iter()
                    .map(|event| EventOpt::from_return_type_optional_or_required(event, events))
                    .collect::<Result<_, _>>()?,
            )),
        }
    }
}

#[cfg(feature = "codegen")]
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

/// Optional or required event from a command.
#[derive(Clone, Debug, PartialEq)]
pub enum EventOpt {
    Optional(Event),
    Required(Event),
}

impl EventOpt {
    fn from_return_type_optional_or_required(
        return_type_optional_or_required: &crate::parser::aggregate::ReturnTypeOptionalOrRequired,
        events: &HashMap<String, Event>,
    ) -> Result<Self, Error> {
        match return_type_optional_or_required {
            crate::parser::aggregate::ReturnTypeOptionalOrRequired::Optional(name) => {
                Ok(EventOpt::Optional(
                    events
                        .get(*name)
                        .ok_or_else(|| Error::EventNotDefined(name.to_string()))?
                        .clone(),
                ))
            }
            crate::parser::aggregate::ReturnTypeOptionalOrRequired::Required(name) => {
                Ok(EventOpt::Required(
                    events
                        .get(*name)
                        .ok_or_else(|| Error::EventNotDefined(name.to_string()))?
                        .clone(),
                ))
            }
        }
    }
}

#[cfg(feature = "codegen")]
impl ToRustType for EventOpt {
    fn to_rust_type(&self) -> String {
        match self {
            EventOpt::Optional(event) => format!("std::option::Option<{}Event>", event.name),
            EventOpt::Required(event) => format!("{}Event", event.name),
        }
    }
}

/// Event definition with name and fields.
///
/// ```text
/// event WithdrewFunds {
///   amount: Float!
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub name: String,
    pub fields: HashMap<String, RepeatableType>,
}

impl Event {
    fn from_event(
        custom_types: &HashMap<&str, &Vec<crate::parser::event::Field>>,
        event: &crate::parser::event::Event,
    ) -> Result<Self, Error> {
        let name = event.ident.to_string();
        let fields = event
            .fields
            .iter()
            .try_fold(HashMap::new(), |mut acc, field| {
                if acc
                    .insert(
                        field.ident.to_string(),
                        RepeatableType::from_type(custom_types, &field.ty)?,
                    )
                    .is_some()
                {
                    return Err(Error::DuplicateEventField {
                        event: name.clone(),
                        field: field.ident.to_string(),
                    });
                }

                Ok(acc)
            })?;

        Ok(Event { name, fields })
    }
}

/// Custom type definition with name and fields.
///
/// ```text
/// type User {
///   name: String!
///   age: Int
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct CustomType {
    pub name: String,
    pub fields: HashMap<String, RepeatableType>,
}

impl CustomType {
    fn from_custom_type(
        custom_types: &HashMap<&str, &Vec<crate::parser::event::Field>>,
        ty: &crate::parser::types::CustomType,
    ) -> Result<Self, Error> {
        let name = ty.ident.to_string();
        let fields = ty
            .fields
            .iter()
            .try_fold(HashMap::new(), |mut acc, field| {
                if acc
                    .insert(
                        field.ident.to_string(),
                        RepeatableType::from_type(custom_types, &field.ty)?,
                    )
                    .is_some()
                {
                    return Err(Error::DuplicateCustomTypeField {
                        ty: name.clone(),
                        field: field.ident.to_string(),
                    });
                }

                Ok(acc)
            })?;

        Ok(CustomType { name, fields })
    }
}

/// A type which can be a single type or array type.
/// - `String`
/// - `[String]`
/// - `[String]!`
#[derive(Clone, Debug, PartialEq)]
pub enum RepeatableType {
    Single(TypeOpt),
    OptionalArray(TypeOpt),
    RequiredArray(TypeOpt),
}

impl RepeatableType {
    fn from_type(
        custom_types: &HashMap<&str, &Vec<crate::parser::event::Field>>,
        ty: &crate::parser::types::Type,
    ) -> Result<Self, Error> {
        match ty {
            crate::parser::types::Type::Single(type_opt) => Ok(RepeatableType::Single(
                TypeOpt::from_optional_or_required_type(custom_types, type_opt)?,
            )),
            crate::parser::types::Type::Array {
                inner,
                required: false,
            } => Ok(RepeatableType::OptionalArray(
                TypeOpt::from_optional_or_required_type(custom_types, inner)?,
            )),
            crate::parser::types::Type::Array {
                inner,
                required: true,
            } => Ok(RepeatableType::RequiredArray(
                TypeOpt::from_optional_or_required_type(custom_types, inner)?,
            )),
        }
    }
}

#[cfg(feature = "codegen")]
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

/// An optional or required type.
/// - `String`
/// - `String!`
#[derive(Clone, Debug, PartialEq)]
pub enum TypeOpt {
    Optional(TypeRef),
    Required(TypeRef),
}

impl TypeOpt {
    fn from_optional_or_required_type(
        custom_types: &HashMap<&str, &Vec<crate::parser::event::Field>>,
        ty: &crate::parser::types::OptionalOrRequiredType,
    ) -> Result<Self, Error> {
        match ty {
            crate::parser::types::OptionalOrRequiredType::Optional(type_ref) => Ok(
                TypeOpt::Optional(TypeRef::from_scalar_or_user_type(custom_types, type_ref)?),
            ),
            crate::parser::types::OptionalOrRequiredType::Required(type_ref) => Ok(
                TypeOpt::Required(TypeRef::from_scalar_or_user_type(custom_types, type_ref)?),
            ),
        }
    }
}

#[cfg(feature = "codegen")]
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

/// A type reference of either scalar or custom type.
/// - `Int`
/// - `MyType`
#[derive(Clone, Debug, PartialEq)]
pub enum TypeRef {
    Scalar(Scalar),
    Custom(CustomType),
}

impl TypeRef {
    fn from_scalar_or_user_type(
        custom_types: &HashMap<&str, &Vec<crate::parser::event::Field>>,
        type_ref: &crate::parser::types::ScalarOrUserType,
    ) -> Result<Self, Error> {
        match type_ref {
            crate::parser::types::ScalarOrUserType::Scalar(scalar) => {
                Ok(TypeRef::Scalar((*scalar).into()))
            }
            crate::parser::types::ScalarOrUserType::UserDefined(custom) => {
                let fields = custom_types
                    .get(custom)
                    .ok_or_else(|| Error::TypeNotDefined(custom.to_string()))?
                    .iter()
                    .try_fold(HashMap::new(), |mut acc, field| {
                        if acc
                            .insert(
                                field.ident.to_string(),
                                RepeatableType::from_type(custom_types, &field.ty)?,
                            )
                            .is_some()
                        {
                            return Err(Error::DuplicateTypeField {
                                ty: custom.to_string(),
                                field: field.ident.to_string(),
                            });
                        }

                        Ok(acc)
                    })?;

                Ok(TypeRef::Custom(CustomType {
                    name: custom.to_string(),
                    fields,
                }))
            }
        }
    }
}

#[cfg(feature = "codegen")]
impl ToRustType for TypeRef {
    fn to_rust_type(&self) -> String {
        match self {
            TypeRef::Scalar(scalar) => scalar.to_rust_type(),
            TypeRef::Custom(custom_type) => custom_type.name.clone(),
        }
    }
}

/// An in-built scalar type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scalar {
    String,
    Int,
    Float,
    Bool,
}

#[cfg(feature = "codegen")]
impl ToRustType for Scalar {
    fn to_rust_type(&self) -> String {
        match self {
            Scalar::String => "String".to_string(),
            Scalar::Int => "i64".to_string(),
            Scalar::Float => "f64".to_string(),
            Scalar::Bool => "bool".to_string(),
        }
    }
}

impl From<crate::parser::types::Scalar> for Scalar {
    fn from(scalar: crate::parser::types::Scalar) -> Self {
        match scalar {
            crate::parser::types::Scalar::String => Scalar::String,
            crate::parser::types::Scalar::Int => Scalar::Int,
            crate::parser::types::Scalar::Float => Scalar::Float,
            crate::parser::types::Scalar::Bool => Scalar::Bool,
        }
    }
}
