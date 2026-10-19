package oxide.lambdas_and_method_refs;

import java.util.function.Function;
import java.util.function.Predicate;

public class Lambdas {
    /** True for words longer than n characters. */
    public static Predicate<String> longerThan(int n) {
        return s -> s.length() > n;
    }

    /** A word to its length, written as a lambda. */
    public static Function<String, Integer> lengthByLambda() {
        return s -> s.length();
    }

    /** A word to its length, written as a method reference. */
    public static Function<String, Integer> lengthByMethodRef() {
        return String::length;
    }
}
