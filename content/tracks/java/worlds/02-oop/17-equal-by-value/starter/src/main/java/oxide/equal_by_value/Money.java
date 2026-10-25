package oxide.equal_by_value;

public class Money {
    private final long cents;
    private final String currency;

    public Money(long cents, String currency) {
        this.cents = cents;
        this.currency = currency;
    }

    public long cents() {
        return cents;
    }

    public String currency() {
        return currency;
    }

    @Override
    public boolean equals(Object o) {
        // TODO: false unless o is a Money; then equal when cents and currency both match
        return true;
    }
}
