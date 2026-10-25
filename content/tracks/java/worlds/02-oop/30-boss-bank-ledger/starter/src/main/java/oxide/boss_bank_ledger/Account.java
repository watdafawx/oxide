package oxide.boss_bank_ledger;

import java.util.Arrays;

public class Account {
    private final String owner;
    private long balance;
    private Transaction[] history = new Transaction[0];

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
    public Transaction[] history() {
        return history.clone();
    }

    /** Adds one transaction to the end of the history. Written for you: call it, do not change it. */
    private void record(Transaction tx) {
        history = Arrays.copyOf(history, history.length + 1);
        history[history.length - 1] = tx;
    }

    /** Applies one transaction. A rejected one throws OverdraftException and changes nothing. */
    public void apply(Transaction tx) {
        throw new UnsupportedOperationException("TODO: a switch statement over the three kinds of transaction, then record(tx) once it has succeeded");
    }

    /** One numbered line per transaction, then "balance 50.00". Lines are separated by \n. */
    public String statement() {
        throw new UnsupportedOperationException("TODO: build the statement with a StringBuilder");
    }

    /** 1250 -> "12.50". */
    static String money(long cents) {
        return String.format("%d.%02d", cents / 100, cents % 100);
    }
}
