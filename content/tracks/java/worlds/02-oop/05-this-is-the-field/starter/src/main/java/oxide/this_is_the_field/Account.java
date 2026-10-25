package oxide.this_is_the_field;

public class Account {
    private String owner;
    private long balance;

    public Account(String owner, long balance) {
        // TODO: these two lines copy each parameter onto itself; make the left side the field
        owner = owner;
        balance = balance;
    }

    public String owner() {
        return owner;
    }

    /** Balance in cents. */
    public long balance() {
        return balance;
    }
}
