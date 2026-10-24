package oxide.data_in_one_line;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ParcelTest {
    @Test
    void parcelIsARecord() {
        assertThat(Parcel.class.isRecord()).as("declare Parcel with `record`").isTrue();
        Parcel books = new Parcel("books", 1200);
        assertThat(books.label()).isEqualTo("books");
        assertThat(books.grams()).isEqualTo(1200);
    }

    @Test
    void recordEqualsByValue() {
        Parcel a = new Parcel("books", 1200);
        assertThat(a.equals(new Parcel("books", 1200))).isTrue();
        assertThat(a.equals(new Parcel("books", 1201))).isFalse();
        assertThat(a.hashCode()).isEqualTo(new Parcel("books", 1200).hashCode());
        assertThat(a.toString()).isEqualTo("Parcel[label=books, grams=1200]");
    }
}
