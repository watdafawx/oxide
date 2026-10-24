package oxide.hash_matches_equals;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.HashSet;
import java.util.Set;
import org.junit.jupiter.api.Test;

class MoneyTest {
    @Test
    void equalMoneyHasEqualHashCodes() {
        Money a = new Money(1250, "EUR");
        Money b = new Money(1250, "EUR");
        assertThat(a.equals(b)).isTrue();
        assertThat(a.hashCode()).as("equal objects must have equal hash codes").isEqualTo(b.hashCode());
    }

    @Test
    void hashSetFindsEqualMoney() {
        Set<Money> prices = new HashSet<>();
        prices.add(new Money(1250, "EUR"));
        assertThat(prices.contains(new Money(1250, "EUR"))).as("the set finds an equal amount").isTrue();
        prices.add(new Money(1250, "EUR"));
        assertThat(prices).as("a set holds a value once").hasSize(1);
    }
}
