package oxide.reject_bad_values;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class ParcelTest {
    @Test
    void negativeGramsAreRejected() {
        assertThatThrownBy(() -> new Parcel("books", -1)).isInstanceOf(IllegalArgumentException.class);
        assertThat(new Parcel("letter", 0).grams()).as("0 grams is fine").isZero();
    }

    @Test
    void labelIsTrimmed() {
        assertThat(new Parcel("  books ", 1200).label()).isEqualTo("books");
    }
}
