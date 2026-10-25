package oxide.set_once;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.reflect.Modifier;
import org.junit.jupiter.api.Test;

class AccountTest {
    @Test
    void ownerIsFinal() throws Exception {
        int modifiers = Account.class.getDeclaredField("owner").getModifiers();
        assertThat(Modifier.isFinal(modifiers)).as("write `private final String owner;`").isTrue();
    }

    @Test
    void depositAddsToBalance() throws Exception {
        Account ann = new Account("Ann");
        ann.deposit(500);
        ann.deposit(250);
        assertThat(ann.balance()).isEqualTo(750);
        int modifiers = Account.class.getDeclaredField("balance").getModifiers();
        assertThat(Modifier.isFinal(modifiers)).as("the balance changes, so it is not final").isFalse();
    }
}
