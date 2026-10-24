package oxide.method_references;

import java.util.function.Function;

public class Names {
    /** A mod to its name, written as a lambda. */
    public static Function<Mod, String> nameByLambda() {
        return mod -> mod.name();
    }

    /** A mod to its name, written as a method reference. */
    public static Function<Mod, String> nameByReference() {
        return Mod::name;
    }
}
