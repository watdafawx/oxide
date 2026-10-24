package oxide.private_means_mine;

public class PiggyBank {
    private int coins;

    /** Puts coins in. */
    public void add(int amount) {
        coins = coins + amount;
    }

    /** How many coins are inside. */
    public int coins() {
        return coins;
    }

    /** Takes coins out when there are enough. Hands back whether it did. */
    public boolean take(int amount) {
        if (amount > coins) {
            return false;
        }
        coins = coins - amount;
        return true;
    }
}
