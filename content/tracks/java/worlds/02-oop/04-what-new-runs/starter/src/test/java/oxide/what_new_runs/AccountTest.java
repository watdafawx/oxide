package oxide.what_new_runs;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class AccountTest {
    @Test
    void constructorSetsOwnerAndBalance() {
        Account ann = new Account("Ann", 500);
        assertThat(ann.owner()).isEqualTo("Ann");
        assertThat(ann.balance()).isEqualTo(500);
        Account bo = new Account("Bo", 20);
        assertThat(bo.owner()).isEqualTo("Bo");
        assertThat(bo.balance()).isEqualTo(20);
    }

    @Test
    void negativeOpeningStartsAtZero() {
        Account cy = new Account("Cy", -200);
        assertThat(cy.owner()).isEqualTo("Cy");
        assertThat(cy.balance()).as("an account never opens below 0").isZero();
    }
}
