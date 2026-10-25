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
        switch (tx) {
            case Deposit d -> balance += d.cents();
            case Withdraw w -> take(w.cents());
            case Transfer t -> {
                take(t.cents());
                t.to().apply(new Deposit(t.cents()));
            }
        }
        record(tx);
    }

    private void take(long cents) {
        if (cents > balance) {
            throw new OverdraftException(owner + " cannot spend " + money(cents) + ", balance is " + money(balance));
        }
        balance -= cents;
    }

    /** One numbered line per transaction, then "balance 50.00". Lines are separated by \n. */
    public String statement() {
        StringBuilder out = new StringBuilder();
        int number = 1;
        for (Transaction tx : history) {
            String text = switch (tx) {
                case Deposit d -> "deposit " + money(d.cents());
                case Withdraw w -> "withdraw " + money(w.cents());
                case Transfer t -> "transfer " + money(t.cents()) + " to " + t.to().owner();
            };
            out.append(number).append(". ").append(text).append("\n");
            number++;
        }
        out.append("balance ").append(money(balance));
        return out.toString();
    }

    /** 1250 -> "12.50". */
    static String money(long cents) {
        return String.format("%d.%02d", cents / 100, cents % 100);
    }
}
