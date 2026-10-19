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
        switch (tx) {
            case Deposit d -> balance += d.cents();
            case Withdraw w -> take(w.cents());
            case Transfer t -> {
                take(t.cents());
                t.to().apply(new Deposit(t.cents()));
            }
        }
        history.add(tx);
    }

    private void take(long cents) {
        if (cents > balance) {
            throw new OverdraftException(owner + " cannot spend " + money(cents) + ", balance is " + money(balance));
        }
        balance -= cents;
    }

    /** One numbered line per transaction, then "balance 50.00". Lines are joined with \n. */
    public String statement() {
        List<String> lines = new ArrayList<>();
        int number = 1;
        for (Transaction tx : history) {
            String text = switch (tx) {
                case Deposit d -> "deposit " + money(d.cents());
                case Withdraw w -> "withdraw " + money(w.cents());
                case Transfer t -> "transfer " + money(t.cents()) + " to " + t.to().owner();
            };
            lines.add(number + ". " + text);
            number++;
        }
        lines.add("balance " + money(balance));
        return String.join("\n", lines);
    }

    /** 1250 -> "12.50". */
    static String money(long cents) {
        return String.format("%d.%02d", cents / 100, cents % 100);
    }
}
