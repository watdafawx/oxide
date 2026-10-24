package oxide.check_then_change;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class WalletTest {
    @Test
    void overspendingThrowsAndChangesNothing() {
        Wallet ann = new Wallet("Ann", 1000);
        assertThatThrownBy(() -> ann.spend(1001)).isInstanceOf(NotEnoughMoneyException.class);
        assertThat(ann.cents()).as("a refused spend changes nothing").isEqualTo(1000);
        ann.spend(1000);
        assertThat(ann.cents()).as("spending exactly what is there is fine").isZero();
    }

    @Test
    void refusedPaymentChangesNeitherWallet() {
        Wallet ann = new Wallet("Ann", 1000);
        Wallet bo = new Wallet("Bo", 0);
        assertThatThrownBy(() -> ann.pay(bo, 5000)).isInstanceOf(NotEnoughMoneyException.class);
        assertThat(ann.cents()).isEqualTo(1000);
        assertThat(bo.cents()).as("no money appears from nowhere").isZero();
        ann.pay(bo, 400);
        assertThat(ann.cents()).isEqualTo(600);
        assertThat(bo.cents()).isEqualTo(400);
    }
}
