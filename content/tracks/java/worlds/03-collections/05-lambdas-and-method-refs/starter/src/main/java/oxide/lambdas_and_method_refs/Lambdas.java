package oxide.lambdas_and_method_refs;

import java.util.function.Function;
import java.util.function.Predicate;

public class Lambdas {
    /** True for words longer than n characters. */
    public static Predicate<String> longerThan(int n) {
        throw new UnsupportedOperationException("TODO: return a lambda s -> ... that compares the length with n");
    }

    /** A word to its length, written as a lambda. */
    public static Function<String, Integer> lengthByLambda() {
        throw new UnsupportedOperationException("TODO: return a lambda s -> ...");
    }

    /** A word to its length, written as a method reference. */
    public static Function<String, Integer> lengthByMethodRef() {
        throw new UnsupportedOperationException("TODO: return a method reference to String's length method");
    }
}
