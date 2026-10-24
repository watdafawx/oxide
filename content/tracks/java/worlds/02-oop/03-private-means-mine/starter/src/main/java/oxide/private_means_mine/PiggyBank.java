package oxide.private_means_mine;

public class PiggyBank {
    // TODO: make this field private, so only PiggyBank's own methods can change it
    int coins;

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
        throw new UnsupportedOperationException("TODO: refuse when amount is more than coins, otherwise take them");
    }
}
