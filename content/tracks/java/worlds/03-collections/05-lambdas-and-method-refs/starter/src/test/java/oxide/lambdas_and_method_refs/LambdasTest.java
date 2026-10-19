package oxide.lambdas_and_method_refs;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.function.Predicate;
import org.junit.jupiter.api.Test;

class LambdasTest {
    @Test
    void appliesPredicate() {
        Predicate<String> longerThanThree = Lambdas.longerThan(3);
        assertThat(longerThanThree.test("abcd")).isTrue();
        assertThat(longerThanThree.test("abc")).isFalse();
        assertThat(Lambdas.longerThan(0).test("")).isFalse();
    }

    @Test
    void methodRefEqualsLambda() {
        for (String word : List.of("", "a", "lambda", "method reference")) {
            assertThat(Lambdas.lengthByLambda().apply(word)).isEqualTo(word.length());
            assertThat(Lambdas.lengthByMethodRef().apply(word)).isEqualTo(Lambdas.lengthByLambda().apply(word));
        }
    }
}
