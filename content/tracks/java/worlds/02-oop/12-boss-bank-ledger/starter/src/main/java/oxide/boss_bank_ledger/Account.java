package oxide.boss_bank_ledger;

import java.util.ArrayList;
import java.util.List;

public class Account {
    private final String owner;
    private long balance;
    private final List<Transaction> history = new ArrayList<>();

    public Account(String owner) {
        this.owner = owner;
    }

    public String owner() {
        return owner;
    }

    /** Balance in cents. */
    public long balance() {
        return balance;
    }

    /** A copy of the transactions applied so far, oldest first. */
    public List<Transaction> history() {
        return List.copyOf(history);
    }

    /** Applies one transaction. A rejected one throws and changes nothing. */
    public void apply(Transaction tx) {
        // TODO: switch (tx) over Deposit, Withdraw and Transfer; add tx to history once it has succeeded
        throw new UnsupportedOperationException("TODO: apply the transaction");
    }

    /** One numbered line per transaction, then "balance 50.00". Lines are joined with \n. */
    public String statement() {
        throw new UnsupportedOperationException("TODO: build the statement");
    }

    /** 1250 -> "12.50". */
    static String money(long cents) {
        return String.format("%d.%02d", cents / 100, cents % 100);
    }
}
