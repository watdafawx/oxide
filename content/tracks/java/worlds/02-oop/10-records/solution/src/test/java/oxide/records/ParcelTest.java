package oxide.records;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.util.HashSet;
import java.util.Set;
import org.junit.jupiter.api.Test;

class ParcelTest {
    @Test
    void recordEqualsByValue() {
        assertThat(Parcel.class.isRecord()).as("Parcel should be declared with `record`").isTrue();
        Parcel a = new Parcel("books", 1200);
        assertThat(a).isEqualTo(new Parcel("books", 1200));
        assertThat(a).isNotEqualTo(new Parcel("books", 1201));
        assertThat(a.hashCode()).isEqualTo(new Parcel("books", 1200).hashCode());
        assertThat(a).hasToString("Parcel[label=books, grams=1200]");
        Set<Parcel> set = new HashSet<>(Set.of(a));
        assertThat(set).contains(new Parcel("books", 1200));
    }

    @Test
    void compactConstructorRejectsNegative() {
        assertThatThrownBy(() -> new Parcel("books", -1)).isInstanceOf(IllegalArgumentException.class);
        assertThat(new Parcel("letter", 0).grams()).isZero();
    }

    @Test
    void withMethodReturnsCopy() {
        Parcel original = new Parcel("books", 1200);
        Parcel lighter = original.withGrams(900);
        assertThat(lighter).isEqualTo(new Parcel("books", 900));
        assertThat(original.grams()).as("the original never changes").isEqualTo(1200);
        assertThat(lighter).isNotSameAs(original);
        assertThatThrownBy(() -> original.withGrams(-5)).isInstanceOf(IllegalArgumentException.class);
    }
}
