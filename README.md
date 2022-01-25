# ESDL

**E**vent **S**ourcing Schema **D**efinition **L**anguage

---

Schema definition language for defining aggregates, commands, events & custom types.

Heavily inspired by GraphQL syntax, you can describe aggregates which can be used for codegen in different languages.

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

| Scalar      | Rust Type                                                                                            |
| ----------- | ---------------------------------------------------------------------------------------------------- |
| `String`    | [`String`](https://doc.rust-lang.org/stable/std/string/struct.String.html)                           |
| `Int`       | [`i64`](https://doc.rust-lang.org/stable/std/primitive.i64.html)                                     |
| `Float`     | [`f64`](https://doc.rust-lang.org/stable/std/primitive.f64.html)                                     |
| `Bool`      | [`bool`](https://doc.rust-lang.org/stable/std/primitive.bool.html)                                   |
| `Timestamp` | [`chrono::DateTime<chrono::FixedOffset>`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) |

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
