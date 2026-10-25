package oxide.method_references;

import java.util.function.Function;

public class Lengths {
    /** A word to its length, written as a lambda. */
    public static Function<String, Integer> lengthByLambda() {
        return w -> w.length();
    }

    /** A word to its length, written as a method reference. */
    public static Function<String, Integer> lengthByReference() {
        return String::length;
    }
}
