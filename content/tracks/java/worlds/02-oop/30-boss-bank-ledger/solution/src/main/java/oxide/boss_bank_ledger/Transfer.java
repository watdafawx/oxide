package oxide.boss_bank_ledger;

public record Transfer(Account to, long cents) implements Transaction {}
