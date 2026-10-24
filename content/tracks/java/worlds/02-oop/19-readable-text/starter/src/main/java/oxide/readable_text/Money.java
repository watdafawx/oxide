package oxide.readable_text;

import java.util.Objects;

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

    @Override
    public int hashCode() {
        return Objects.hash(cents, currency);
    }

    @Override
    public String toString() {
        // TODO: "12.50 EUR" for 1250 cents: whole euros, a dot, two-digit cents, a space, the currency
        return super.toString();
    }
}
