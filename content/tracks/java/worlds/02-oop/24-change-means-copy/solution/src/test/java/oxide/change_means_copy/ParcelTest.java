package oxide.change_means_copy;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class ParcelTest {
    @Test
    void withMethodReturnsCopy() {
        Parcel original = new Parcel("books", 1200);
        Parcel lighter = original.withGrams(900);
        assertThat(lighter).isEqualTo(new Parcel("books", 900));
        assertThat(original.grams()).as("the original never changes").isEqualTo(1200);
    }

    @Test
    void copyFollowsTheSameRules() {
        Parcel original = new Parcel("books", 1200);
        assertThatThrownBy(() -> original.withGrams(-5)).isInstanceOf(IllegalArgumentException.class);
    }
}
