package oxide.equal_by_value;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MoneyTest {
    @Test
    void equalsComparesFields() {
        Money price = new Money(1250, "EUR");
        assertThat(price.equals(new Money(1250, "EUR"))).as("same amount, same currency").isTrue();
        assertThat(price.equals(new Money(1251, "EUR"))).isFalse();
        assertThat(price.equals(new Money(1250, "USD"))).isFalse();
    }

    @Test
    void notEqualToOtherThings() {
        Money price = new Money(1250, "EUR");
        assertThat(price.equals(new Money(1250, new String("EUR")))).as("currency compared with equals, not ==").isTrue();
        assertThat(price.equals(null)).isFalse();
        assertThat(price.equals("12.50 EUR")).as("a String is never Money").isFalse();
    }
}
