use nom_supreme::{error::ErrorTree, final_parser::final_parser};

pub mod aggregate;
pub mod event;
pub mod ident;
pub mod parsers;
pub mod schema;
pub mod types;
pub mod version;

pub type Span<'i> = &'i str;
pub type Error<I> = ErrorTree<I>;
pub type IResult<I, O, E = Error<I>> = Result<(I, O), nom::Err<E>>;

use crate::parser::schema::parse_schema;

use self::schema::Schema;

pub fn parse<'i>(input: impl Into<Span<'i>>) -> Result<Schema<'i>, Error<Span<'i>>> {
    final_parser(parse_schema)(input.into())
}

#[cfg(test)]
mod tests {
    use semver::{BuildMetadata, Prerelease, Version};

    use super::{
        aggregate::{Aggregate, Command, Param, ReturnType, ReturnTypeOptionalOrRequired},
        event::{Event, Field},
        parse,
        schema::Schema,
        types::{OptionalOrRequiredType, Scalar, ScalarOrUserType, Type},
        Error, Span,
    };

    #[test]
    fn it_parses_basic_schema() -> Result<(), Error<Span<'static>>> {
        let schema_str = r#"
            version = "0.1.0"

            aggregate Hello {
                world(name: String) -> FooEvent
            }

            event FooEvent {
                name: String
            }
        "#;

        let expected = Schema {
            versions: vec![Version::new(0, 1, 0)],
            aggregates: vec![Aggregate {
                ident: "Hello",
                commands: vec![Command {
                    ident: "world",
                    params: vec![Param {
                        ident: "name",
                        ty: Type::Single(OptionalOrRequiredType::Required(
                            ScalarOrUserType::Scalar(Scalar::String),
                        )),
                    }],
                    return_type: ReturnType::Single(ReturnTypeOptionalOrRequired::Required(
                        "FooEvent",
                    )),
                }],
            }],
            events: vec![Event {
                ident: "FooEvent",
                fields: vec![Field {
                    ident: "name",
                    ty: Type::Single(OptionalOrRequiredType::Required(ScalarOrUserType::Scalar(
                        Scalar::String,
                    ))),
                }],
            }],
            types: vec![],
        };

        assert_eq!(parse(schema_str)?, expected);

        Ok(())
    }

    #[test]
    fn it_parses_bank_account_schema() -> Result<(), Error<Span<'static>>> {
        let schema_str = r#"
          version = "10.2.4-alpha"

          aggregate BankAccount {
            open_account(name: String?, initial_balance: Float) -> OpenedAccount?
            deposit_funds(amount: Float) -> DepositedFunds?
            withdraw_funds(amount: Float) -> WithdrewFunds?
          }
          
          event OpenedAccount {
            name: String?
            initial_balance: Float
          }
          
          event DepositedFunds {
            amount: Float
          }
          
          event WithdrewFunds {
            amount: Float
          }          
        "#;

        let version = Version {
            major: 10,
            minor: 2,
            patch: 4,
            pre: Prerelease::new("alpha").unwrap(),
            build: BuildMetadata::EMPTY,
        };

        let expected = Schema {
            versions: vec![version],
            aggregates: vec![Aggregate {
                ident: "BankAccount",
                commands: vec![
                    Command {
                        ident: "open_account",
                        params: vec![
                            Param {
                                ident: "name",
                                ty: Type::Single(OptionalOrRequiredType::Optional(
                                    ScalarOrUserType::Scalar(Scalar::String),
                                )),
                            },
                            Param {
                                ident: "initial_balance",
                                ty: Type::Single(OptionalOrRequiredType::Required(
                                    ScalarOrUserType::Scalar(Scalar::Float),
                                )),
                            },
                        ],
                        return_type: ReturnType::Single(ReturnTypeOptionalOrRequired::Optional(
                            "OpenedAccount",
                        )),
                    },
                    Command {
                        ident: "deposit_funds",
                        params: vec![Param {
                            ident: "amount",
                            ty: Type::Single(OptionalOrRequiredType::Required(
                                ScalarOrUserType::Scalar(Scalar::Float),
                            )),
                        }],
                        return_type: ReturnType::Single(ReturnTypeOptionalOrRequired::Optional(
                            "DepositedFunds",
                        )),
                    },
                    Command {
                        ident: "withdraw_funds",
                        params: vec![Param {
                            ident: "amount",
                            ty: Type::Single(OptionalOrRequiredType::Required(
                                ScalarOrUserType::Scalar(Scalar::Float),
                            )),
                        }],
                        return_type: ReturnType::Single(ReturnTypeOptionalOrRequired::Optional(
                            "WithdrewFunds",
                        )),
                    },
                ],
            }],
            events: vec![
                Event {
                    ident: "OpenedAccount",
                    fields: vec![
                        Field {
                            ident: "name",
                            ty: Type::Single(OptionalOrRequiredType::Optional(
                                ScalarOrUserType::Scalar(Scalar::String),
                            )),
                        },
                        Field {
                            ident: "initial_balance",
                            ty: Type::Single(OptionalOrRequiredType::Required(
                                ScalarOrUserType::Scalar(Scalar::Float),
                            )),
                        },
                    ],
                },
                Event {
                    ident: "DepositedFunds",
                    fields: vec![Field {
                        ident: "amount",
                        ty: Type::Single(OptionalOrRequiredType::Required(
                            ScalarOrUserType::Scalar(Scalar::Float),
                        )),
                    }],
                },
                Event {
                    ident: "WithdrewFunds",
                    fields: vec![Field {
                        ident: "amount",
                        ty: Type::Single(OptionalOrRequiredType::Required(
                            ScalarOrUserType::Scalar(Scalar::Float),
                        )),
                    }],
                },
            ],
            types: vec![],
        };

        assert_eq!(parse(schema_str)?, expected);

        Ok(())
    }
}
