package oxide.hash_matches_equals;

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
        // TODO: use the same fields as equals, with Objects.hash(...)
        return System.identityHashCode(this);
    }
}
