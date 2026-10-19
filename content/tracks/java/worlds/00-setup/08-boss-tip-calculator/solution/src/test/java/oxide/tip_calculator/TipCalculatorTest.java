package oxide.tip_calculator;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.within;

class TipCalculatorTest {
    @Test
    void parsesAmount() {
        assertThat(TipCalculator.parseAmount("42.50")).isEqualTo(42.5);
        assertThat(TipCalculator.parseAmount("  8 ")).isEqualTo(8.0);
    }

    @Test
    void appliesTipPercent() {
        assertThat(TipCalculator.withTip(40.0, 15)).isCloseTo(46.0, within(0.001));
        assertThat(TipCalculator.withTip(10.0, 0)).isCloseTo(10.0, within(0.001));
    }

    @Test
    void splitsEvenly() {
        assertThat(TipCalculator.splitEvenly(46.0, 4)).isCloseTo(11.5, within(0.001));
        assertThat(TipCalculator.splitEvenly(10.0, 3)).isCloseTo(3.3333, within(0.001));
    }

    @Test
    void receiptIsFormatted() {
        assertThat(TipCalculator.receipt("40.00", 15, 4))
                .isEqualTo("Bill: $40.00\nTip (15%): $6.00\nTotal: $46.00\nEach (4): $11.50");
        assertThat(TipCalculator.receipt("10", 0, 3))
                .isEqualTo("Bill: $10.00\nTip (0%): $0.00\nTotal: $10.00\nEach (3): $3.33");
    }
}
