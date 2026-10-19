package oxide.equals_hashcode_tostring;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.HashSet;
import java.util.Set;
import org.junit.jupiter.api.Test;

class MoneyTest {
    @Test
    void equalsComparesFields() {
        Money a = new Money(1250, "EUR");
        assertThat(a).isEqualTo(new Money(1250, "EUR"));
        assertThat(a).isNotEqualTo(new Money(1251, "EUR"));
        assertThat(a).isNotEqualTo(new Money(1250, "USD"));
        assertThat(a).isNotEqualTo(null);
        assertThat(a).isNotEqualTo("12.50 EUR");
    }

    @Test
    void hashCodeMatchesEquals() {
        Money a = new Money(1250, "EUR");
        Money b = new Money(1250, "EUR");
        assertThat(a).isEqualTo(b);
        assertThat(a.hashCode()).isEqualTo(b.hashCode());

        Set<Money> prices = new HashSet<>();
        prices.add(a);
        assertThat(prices).contains(b);
        prices.add(b);
        assertThat(prices).as("a set holds a value once").hasSize(1);
    }

    @Test
    void toStringIsReadable() {
        assertThat(new Money(1250, "EUR")).hasToString("12.50 EUR");
        assertThat(new Money(5, "USD")).hasToString("0.05 USD");
    }
}
