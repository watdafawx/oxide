Money code is where "it mostly works" is not enough. Every kind of transaction must be handled, a refused one must leave no trace, and the books must add up. This boss uses the whole world: a class with private fields, records, a sealed interface, a switch statement, and check-then-change.

**The ledger.** An `Account` has an owner, a balance in cents (a `long`, so there is no rounding), and a history. You change it only by applying a **Transaction**, one of three records: `Deposit(long cents)`, `Withdraw(long cents)` and `Transfer(Account to, long cents)`. They and `OverdraftException` are written.

**Seal it first.** `Transaction` is a plain interface in the starter. Seal it to the three records before you write any switch: the first objective turns green, and from then on the compiler counts your cases.

**The rules.** A deposit adds to the balance. A withdrawal takes away, but never below 0: asking for more throws `OverdraftException`, with the balance and history unchanged. A transfer is a withdrawal here plus a deposit on `to`; the receiver records an ordinary `Deposit`, the sender records the `Transfer`.

**No trace when refused.** Check before you change, as with the wallet. The private helper `record(tx)` adds to the history; call it after the switch, so a `throw` skips it. In a transfer, take from the sender first, so a refusal never reaches the receiver.

**The statement.** One numbered line per transaction, oldest first, then the balance. `money(cents)` is written and turns 1250 into `12.50`:

```text
1. deposit 100.00
2. withdraw 30.00
3. transfer 20.00 to Bob
balance 50.00
```

**Work in order.** The objectives go from easy to hard, and each one builds on the one before. The hints follow the same order.

## Your turn

- Seal `Transaction` to `Deposit`, `Withdraw` and `Transfer`.
- Write `Account.apply`: a switch statement over the three kinds, then `record(tx)`.
- Write `Account.statement()` with a `StringBuilder`.
