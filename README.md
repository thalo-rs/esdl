# ESDL

**E**vent-sourcing **S**chema **D**efinition **L**anguage

---

Schema definition language for defining aggregates, commands, events & custom types.

Heavily inspired by GraphQL syntax, you can describe aggregates which can be used for codegen in different languages.

## Install

> Due to the wit-bindgen project not having their crates published yet (see https://github.com/bytecodealliance/wit-bindgen/issues/180),
> `esdl` is not up to date on crates.io and it's advised to use the git url as the dependency for now.

```toml
esdl = { git = "https://github.com/thalo-rs/esdl", features = ["codegen-rust"] }
```

_Possible features include `codegen-rust`, `codegen-rust-wasm` and `codegen-typescript`._

## Code generation

ESDL schemas can be used for code generation.

The [Rust crate](https://crates.io/crates/esdl) currently supports code generation for:

- [Rust](https://docs.rs/esdl/latest/esdl/codegen/rust/struct.RustCompiler.html) (`codegen-rust`)
- [Rust WASM](https://docs.rs/esdl/latest/esdl/codegen/rust/wasm/struct.RustWasmCompiler.html) (`codegen-rust-wasm`)
- [TypeScript](https://docs.rs/esdl/latest/esdl/codegen/typescript/struct.TypeScriptCompiler.html) (`codegen-typescript`)

Additional languages may be added in the future. Contributions are welcome!

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

| Scalar      | Rust Type                 | TypeScript Type |
| ----------- | ------------------------- | --------------- |
| `String`    | [`String`]                | [`string`][ts]  |
| `Int`       | [`i64`]                   | [`number`][ts]  |
| `UInt`      | [`u64`]                   | [`number`][ts]  |
| `Float`     | [`f64`]                   | [`number`][ts]  |
| `Bool`      | [`bool`]                  | [`boolean`][ts] |
| `Timestamp` | [`DateTime<FixedOffset>`] | [`Date`]        |

[`string`]: https://doc.rust-lang.org/stable/std/string/struct.String.html
[`i64`]: https://doc.rust-lang.org/stable/std/primitive.i64.html
[`u64`]: https://doc.rust-lang.org/stable/std/primitive.u64.html
[`f64`]: https://doc.rust-lang.org/stable/std/primitive.f64.html
[`bool`]: https://doc.rust-lang.org/stable/std/primitive.bool.html
[`datetime<fixedoffset>`]: https://docs.rs/chrono/latest/chrono/struct.DateTime.html
[ts]: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean
[`boolean`]: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean
[`date`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date

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
