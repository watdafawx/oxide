package oxide.method_references;

import java.util.function.Function;

public class Lengths {
    /** A word to its length, written as a lambda. */
    public static Function<String, Integer> lengthByLambda() {
        // TODO: return a lambda w -> ... that gives the length
        return null;
    }

    /** A word to its length, written as a method reference. */
    public static Function<String, Integer> lengthByReference() {
        // TODO: return a method reference to String's length method
        return null;
    }
}
