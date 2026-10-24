package oxide.set_once;

public class Account {
    // TODO: the owner never changes after the constructor: make it final
    private String owner;
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
        // TODO: add cents to this account's balance
    }
}
