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

---

Integrates with [Thalo](https://github.com/thalo-rs/thalo) to generate Rust code.
