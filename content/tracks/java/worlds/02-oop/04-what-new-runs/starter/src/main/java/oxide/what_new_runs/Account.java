package oxide.what_new_runs;

public class Account {
    private String owner;
    private long balance;

    public Account(String name, long opening) {
        // TODO: store name in owner, and opening in balance (but 0 when opening is below 0)
    }

    public String owner() {
        return owner;
    }

    /** Balance in cents. */
    public long balance() {
        return balance;
    }
}
