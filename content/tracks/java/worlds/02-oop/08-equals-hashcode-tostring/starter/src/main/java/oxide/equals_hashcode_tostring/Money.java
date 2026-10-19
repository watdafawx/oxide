package oxide.equals_hashcode_tostring;

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
        // TODO: equal when it is a Money with the same cents and currency
        return this == o;
    }

    @Override
    public int hashCode() {
        // TODO: use the same fields as equals, e.g. Objects.hash(...)
        return System.identityHashCode(this);
    }

    @Override
    public String toString() {
        // TODO: "12.50 EUR" for 1250 cents
        return super.toString();
    }
}
