package oxide.readable_text;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MoneyTest {
    @Test
    void toStringIsReadable() {
        assertThat(new Money(1250, "EUR").toString()).isEqualTo("12.50 EUR");
        assertThat(new Money(5, "USD").toString()).as("cents always have two digits").isEqualTo("0.05 USD");
    }

    @Test
    void joiningTextUsesToString() {
        Money price = new Money(990, "EUR");
        assertThat("Total: " + price).isEqualTo("Total: 9.90 EUR");
    }
}
