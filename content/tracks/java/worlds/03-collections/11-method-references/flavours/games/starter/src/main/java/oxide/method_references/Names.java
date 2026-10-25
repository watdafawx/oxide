package oxide.method_references;

import java.util.function.Function;

public class Names {
    /** A mod to its name, written as a lambda. */
    public static Function<Mod, String> nameByLambda() {
        // TODO: return a lambda mod -> ... that gives the name
        return null;
    }

    /** A mod to its name, written as a method reference. */
    public static Function<Mod, String> nameByReference() {
        // TODO: return a method reference to Mod's name accessor
        return null;
    }
}
