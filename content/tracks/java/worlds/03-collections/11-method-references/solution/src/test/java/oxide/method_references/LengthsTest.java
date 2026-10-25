package oxide.method_references;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class LengthsTest {
    private final List<String> words = List.of("", "a", "lambda", "method reference");

    @Test
    void lengthByLambdaGivesTheLength() {
        for (String w : words) {
            assertThat(Lengths.lengthByLambda().apply(w)).isEqualTo(w.length());
        }
    }

    @Test
    void lengthByReferenceMatchesTheLambda() {
        for (String w : words) {
            assertThat(Lengths.lengthByReference().apply(w)).isEqualTo(w.length());
        }
    }
}
