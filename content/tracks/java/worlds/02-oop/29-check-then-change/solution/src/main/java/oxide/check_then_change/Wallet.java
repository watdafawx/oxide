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
        if (amount > cents) {
            throw new NotEnoughMoneyException(owner + " has " + cents + " cents, not " + amount);
        }
        cents -= amount;
    }

    /** Moves cents to another wallet. A refused payment changes neither wallet. */
    public void pay(Wallet to, long amount) {
        spend(amount);
        to.cents += amount;
    }
}
