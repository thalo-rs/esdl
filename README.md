# ESDL

**E**vent-sourcing **S**chema **D**efinition **L**anguage

---

Schema definition language for defining aggregates, commands, events & custom types.

Heavily inspired by GraphQL syntax, you can describe aggregates which can be used for codegen in different languages.

## Install

```toml
esdl = "*"
```

## Example

```
version = "0.1.0"

aggregate BankAccount {
  open_account(initial_balance: Float) -> OpenedAccount
  deposit_funds(amount: Float) -> ReceivedFunds
  withdraw_funds(amount: Float) -> SentFunds
  send_funds(amount: Float, user: User) -> (SentFunds? | ReceivedFunds?)
}

event OpenedAccount {
  initial_balance: Float
}

event SentFunds {
  amount: Float
  user: User?
}

event ReceivedFunds {
  amount: Float
  user: User?
}

type User {
  id: String
  name: String?
}
```

### Scalar Types

| Scalar   | Rust Type   | TypeScript Type |
| -------- | ----------- | --------------- |
| `String` | [`String`]  | [`string`][ts]  |
| `Int`    | [`i32`]     | [`number`][ts]  |
| `Long`   | [`i64`]     | [`number`][ts]  |
| `Float`  | [`f32`]     | [`number`][ts]  |
| `Double` | [`f64`]     | [`number`][ts]  |
| `Bool`   | [`bool`]    | [`boolean`][ts] |
| `Bytes`  | [`Vec<u8>`] | [`string`][ts]  |

[`string`]: https://doc.rust-lang.org/stable/std/string/struct.String.html
[`i32`]: https://doc.rust-lang.org/stable/std/primitive.i32.html
[`i64`]: https://doc.rust-lang.org/stable/std/primitive.i64.html
[`f32`]: https://doc.rust-lang.org/stable/std/primitive.f32.html
[`f64`]: https://doc.rust-lang.org/stable/std/primitive.f64.html
[`bool`]: https://doc.rust-lang.org/stable/std/primitive.bool.html
[`vec<u8>`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
[ts]: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean

### Optional & Required

Types can be marked as optional by adding the `?` suffix.

| Type     | Syntax | Example   |
| -------- | ------ | --------- |
| Required | `T`    | `String`  |
| Optional | `T?`   | `String?` |

### Repeating Types

Types can be repeated by wrapping them in `[]`.

| Type   | Syntax | Example    |
| ------ | ------ | ---------- |
| Single | `T`    | `String`   |
| Array  | `[T]`  | `[String]` |

Remember, we can mark types as optional, even in arrays.

| Type                 | Syntax  | Example      |
| -------------------- | ------- | ------------ |
| Optional Array       | `[T?]?` | `[String?]?` |
| Required Array       | `[T?]`  | `[String?]`  |
| Required Array Items | `[T]?`  | `[String]?`  |
| Required Array Items | `[T]`   | `[String]`   |

---

Integrates with [Thalo](https://github.com/thalo-rs/thalo) to generate Rust code.
