# ESDL

**E**vent-sourcing **S**chema **D**efinition **L**anguage

---

Schema definition language for defining aggregates, commands, events & custom types.

Heavily inspired by GraphQL syntax, you can describe aggregates which can be used for codegen in different languages.

## Code generation

ESDL schemas can be used for code generation.

The [Rust crate](https://crates.io/crates/esdl) currently supports code generation for:

- [Rust](https://docs.rs/esdl/latest/esdl/codegen/rust/struct.RustCompiler.html)
- [TypeScript](https://docs.rs/esdl/latest/esdl/codegen/typescript/struct.TypeScriptCompiler.html)

Additional languages may be added in the future. Contributions are welcome!

## Example

```
aggregate BankAccount {
  open_account(initial_balance: Float!) OpenedAccount!
  deposit_funds(amount: Float!): ReceivedFunds!
  withdraw_funds(amount: Float!): SentFunds!
  transact(amount: Float!, user: User!) (SentFunds | ReceivedFunds)
}

event OpenedAccount {
  initial_balance: Float!
}

event SentFunds {
  amount: Float!
  user: User
}

event ReceivedFunds {
  amount: Float!
  user: User
}

type User {
  id: String!
  name: String
}
```

### Scalar Types

| Scalar      | Rust Type                                                                                            | TypeScript Type                                                                                                          |
| ----------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `String`    | [`String`](https://doc.rust-lang.org/stable/std/string/struct.String.html)                           | [`string`](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean)  |
| `Int`       | [`i64`](https://doc.rust-lang.org/stable/std/primitive.i64.html)                                     | [`number`](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean)  |
| `Float`     | [`f64`](https://doc.rust-lang.org/stable/std/primitive.f64.html)                                     | [`number`](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean)  |
| `Bool`      | [`bool`](https://doc.rust-lang.org/stable/std/primitive.bool.html)                                   | [`boolean`](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean) |
| `Timestamp` | [`chrono::DateTime<chrono::FixedOffset>`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) | [`Date`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)                     |

### Optional & Required

Types can be marked as required by adding the `!` suffix.

| Type     | Syntax   | Example   |
| -------- | -------- | --------- |
| Optional | _(none)_ | `String`  |
| Required | `!`      | `String!` |

### Repeating Types

Types can be repeated by wrapping them in `[]`.

| Type   | Syntax    | Example    |
| ------ | --------- | ---------- |
| Single | _(none)_  | `String`   |
| Array  | `[`...`]` | `[String]` |

Remember, we can mark types as required, even in arrays.

| Type                 | Syntax     | Example     |
| -------------------- | ---------- | ----------- |
| Optional Array       | `[`...`]`  | `[String]`  |
| Required Array       | `[`...`]!` | `[String]!` |
| Required Array Items | `[`...`!]` | `[String!]` |

---

Integrates with [Thalo](https://github.com/thalo-rs/thalo) to generate Rust code.
