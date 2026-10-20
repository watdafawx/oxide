**Why this matters:** money code is where "it mostly works" is not enough. Every kind of transaction must be handled, a rejected one must leave no trace, and the books must add up. This boss uses the whole world: classes and fields, encapsulation, records, sealed interfaces and pattern `switch`.

You are building a small **ledger**. An `Account` has an owner and a balance (in cents, as a `long`, so no rounding errors). You change it by applying a **Transaction**:

```java
public sealed interface Transaction permits Deposit, Withdraw, Transfer {}
public record Deposit(long cents) implements Transaction {}
public record Withdraw(long cents) implements Transaction {}
public record Transfer(Account to, long cents) implements Transaction {}
```

The rules:

- **Deposit** adds to the balance.
- **Withdraw** subtracts, but never below 0: otherwise `OverdraftException`, balance and history unchanged.
- **Transfer** is a withdrawal here plus a deposit on `to`. If the sender cannot afford it, nothing moves on either side. The `to` account records its own `Deposit` for the transfer; the sender records the `Transfer`.
- `statement()` prints the history, then the balance:

```
1. deposit 100.00
2. withdraw 30.00
3. transfer 20.00 to Bob
balance 50.00
```

The records, the exception and `money(cents)` are written. `Transaction` is a plain interface for now.

## Your turn

Work in the order of the objectives; the first one turns green early.

- `Account.apply(Transaction)`: a `switch` over the three records. Use it as a statement: `case Deposit d -> { ... }` with a block body, since `apply` returns nothing.
- `Account.statement()`.
- Last: make `Transaction` sealed, and check your switches have no `default`.
