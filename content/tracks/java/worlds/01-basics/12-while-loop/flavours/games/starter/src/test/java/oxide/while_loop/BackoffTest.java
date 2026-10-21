package oxide.while_loop;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BackoffTest {
    @Test
    void countsHalvingSteps() {
        assertThat(Backoff.retriesUntilUnder(800, 100)).isEqualTo(3);
        assertThat(Backoff.retriesUntilUnder(1000, 100)).isEqualTo(4);
    }

    @Test
    void alreadyUnderNeedsNoRetries() {
        assertThat(Backoff.retriesUntilUnder(50, 100)).isEqualTo(0);
        assertThat(Backoff.retriesUntilUnder(100, 100)).isEqualTo(0);
    }
}
