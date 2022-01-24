aggregate BankAccount  {
  open_account(user: User!, initial_balance: Float): OpenedAccount
  make_transaction(amount: Float): (DepositedFunds | WithdrewFunds!)
}

event OpenedAccount {
  initial_balance: Float!
}

event DepositedFunds {
  amount: Float!
}

event WithdrewFunds {
  amount: Foo!
}

type User {
  name: String
}

type MyType {
  name: String
  foo: Foo
}

type Foo {
  age: Int
}
