package oxide.boss_bank_ledger;

public sealed interface Transaction permits Deposit, Withdraw, Transfer {}
