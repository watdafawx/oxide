package oxide.an_object_remembers;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class CounterTest {
    @Test
    void startsAtZero() {
        assertThat(new Counter().value()).isZero();
    }

    @Test
    void incrementAddsOne() {
        Counter front = new Counter();
        Counter back = new Counter();
        front.increment();
        front.increment();
        assertThat(front.value()).isEqualTo(2);
        assertThat(back.value()).as("the other counter keeps its own count").isZero();
    }
}
