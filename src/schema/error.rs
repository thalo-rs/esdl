use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("duplicate command {0}")]
    DuplicateCommand(String),
    #[error("duplicate custom type {0}")]
    DuplicateCustomType(String),
    #[error("duplicate field {field} in custom type {ty}")]
    DuplicateCustomTypeField { ty: String, field: String },
    #[error("duplicate event {0}")]
    DuplicateEvent(String),
    #[error("duplicate field {field} in event {event}")]
    DuplicateEventField { event: String, field: String },
    #[error("duplicate param {param} in {command}")]
    DuplicateParam { command: String, param: String },
    #[error("duplicate field {field} in type {ty}")]
    DuplicateTypeField { ty: String, field: String },
    #[error("event not defined {0}")]
    EventNotDefined(String),
    #[error("missing aggregate")]
    MissingAggregate,
    #[error("multiple aggregates")]
    MultipleAggregates,
    #[error("type not defined {0}")]
    TypeNotDefined(String),
}
