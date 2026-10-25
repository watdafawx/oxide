package oxide.check_then_change;

public class Wallet {
    private final String owner;
    private long cents;

    public Wallet(String owner, long cents) {
        this.owner = owner;
        this.cents = cents;
    }

    public long cents() {
        return cents;
    }

    /** Takes cents out. Asking for more than the wallet holds throws and changes nothing. */
    public void spend(long amount) {
        // TODO: throw a NotEnoughMoneyException first when amount is more than cents; only then take it away
        cents -= amount;
    }

    /** Moves cents to another wallet. A refused payment changes neither wallet. */
    public void pay(Wallet to, long amount) {
        // TODO: these two lines are in the wrong order: a refused spend must not leave money in `to`
        to.cents += amount;
        spend(amount);
    }
}
