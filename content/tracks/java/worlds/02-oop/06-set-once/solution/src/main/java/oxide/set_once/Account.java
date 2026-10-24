package oxide.set_once;

public class Account {
    private final String owner;
    private long balance;

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

    /** Adds cents to the balance. */
    public void deposit(long cents) {
        this.balance = this.balance + cents;
    }
}
