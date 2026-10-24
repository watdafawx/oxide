package oxide.this_is_the_field;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class AccountTest {
    @Test
    void ownerIsStored() {
        assertThat(new Account("Ann", 500).owner()).as("the field, not the parameter, must get the value").isEqualTo("Ann");
    }

    @Test
    void balanceIsStored() {
        assertThat(new Account("Ann", 500).balance()).as("the field, not the parameter, must get the value").isEqualTo(500);
    }
}
