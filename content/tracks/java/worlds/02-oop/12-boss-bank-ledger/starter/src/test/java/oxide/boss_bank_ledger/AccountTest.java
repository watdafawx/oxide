package oxide.boss_bank_ledger;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class AccountTest {
    @Test
    void depositIncreasesBalance() {
        Account ann = new Account("Ann");
        ann.apply(new Deposit(10000));
        ann.apply(new Deposit(250));
        assertThat(ann.balance()).isEqualTo(10250);
        assertThat(ann.history()).containsExactly(new Deposit(10000), new Deposit(250));
    }

    @Test
    void overdraftRejected() {
        Account ann = new Account("Ann");
        Account bob = new Account("Bob");
        ann.apply(new Deposit(1000));

        assertThatThrownBy(() -> ann.apply(new Withdraw(1001))).isInstanceOf(OverdraftException.class);
        assertThatThrownBy(() -> ann.apply(new Transfer(bob, 1001))).isInstanceOf(OverdraftException.class);

        assertThat(ann.balance()).as("a rejected transaction changes nothing").isEqualTo(1000);
        assertThat(bob.balance()).isZero();
        assertThat(ann.history()).as("and leaves no history").containsExactly(new Deposit(1000));
        assertThat(bob.history()).isEmpty();

        ann.apply(new Withdraw(1000));
        assertThat(ann.balance()).as("spending exactly the balance is fine").isZero();
    }

    @Test
    void transferMovesMoneyBothWays() {
        Account ann = new Account("Ann");
        Account bob = new Account("Bob");
        ann.apply(new Deposit(5000));
        ann.apply(new Transfer(bob, 2000));
        assertThat(ann.balance()).isEqualTo(3000);
        assertThat(bob.balance()).isEqualTo(2000);
        assertThat(bob.history()).containsExactly(new Deposit(2000));
    }

    @Test
    void statementListsInOrder() {
        Account ann = new Account("Ann");
        Account bob = new Account("Bob");
        ann.apply(new Deposit(10000));
        ann.apply(new Withdraw(3000));
        ann.apply(new Transfer(bob, 2000));
        assertThat(ann.statement()).isEqualTo(
                "1. deposit 100.00\n2. withdraw 30.00\n3. transfer 20.00 to Bob\nbalance 50.00");
        assertThat(bob.statement()).isEqualTo("1. deposit 20.00\nbalance 20.00");
        assertThat(new Account("Cy").statement()).isEqualTo("balance 0.00");
    }

    @Test
    void sealedSwitchIsExhaustive() {
        assertThat(Transaction.class.isSealed()).as("make Transaction `sealed ... permits ...`").isTrue();
        assertThat(Transaction.class.getPermittedSubclasses())
                .containsExactlyInAnyOrder(Deposit.class, Withdraw.class, Transfer.class);
    }
}
