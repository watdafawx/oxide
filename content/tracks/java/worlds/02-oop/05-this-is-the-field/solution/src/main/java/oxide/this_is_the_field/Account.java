package oxide.this_is_the_field;

public class Account {
    private String owner;
    private long balance;

    public Account(String owner, long balance) {
        this.owner = owner;
        this.balance = balance;
    }

    public String owner() {
        return owner;
    }

    /** Balance in cents. */
    public long balance() {
        return balance;
    }
}
