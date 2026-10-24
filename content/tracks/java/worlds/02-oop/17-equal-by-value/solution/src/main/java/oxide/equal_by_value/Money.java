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
        if (!(o instanceof Money other)) {
            return false;
        }
        return cents == other.cents && currency.equals(other.currency);
    }
}
