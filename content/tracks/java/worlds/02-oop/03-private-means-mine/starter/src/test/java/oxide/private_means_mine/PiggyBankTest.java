package oxide.private_means_mine;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.reflect.Modifier;
import org.junit.jupiter.api.Test;

class PiggyBankTest {
    @Test
    void coinsFieldIsPrivate() throws Exception {
        int modifiers = PiggyBank.class.getDeclaredField("coins").getModifiers();
        assertThat(Modifier.isPrivate(modifiers)).as("write `private int coins;`").isTrue();
    }

    @Test
    void takeNeverGoesBelowZero() {
        PiggyBank bank = new PiggyBank();
        bank.add(10);
        assertThat(bank.take(4)).isTrue();
        assertThat(bank.coins()).isEqualTo(6);
        assertThat(bank.take(7)).as("only 6 inside").isFalse();
        assertThat(bank.coins()).as("a refused take changes nothing").isEqualTo(6);
        assertThat(bank.take(6)).as("taking exactly what is there is fine").isTrue();
        assertThat(bank.coins()).isZero();
    }
}
