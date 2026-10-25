package oxide.what_new_runs;

public class Account {
    private String owner;
    private long balance;

    public Account(String name, long opening) {
        owner = name;
        if (opening < 0) {
            balance = 0;
        } else {
            balance = opening;
        }
    }

    public String owner() {
        return owner;
    }

    /** Balance in cents. */
    public long balance() {
        return balance;
    }
}
