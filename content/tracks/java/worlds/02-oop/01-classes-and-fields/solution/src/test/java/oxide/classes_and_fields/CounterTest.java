package oxide.classes_and_fields;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class CounterTest {
    @Test
    void startsAtZero() {
        assertThat(new Counter().value()).isZero();
    }

    @Test
    void incrementAddsOne() {
        Counter counter = new Counter();
        counter.increment();
        counter.increment();
        assertThat(counter.value()).isEqualTo(2);
        assertThat(new Counter().value()).as("other counters are unaffected").isZero();
    }
}
