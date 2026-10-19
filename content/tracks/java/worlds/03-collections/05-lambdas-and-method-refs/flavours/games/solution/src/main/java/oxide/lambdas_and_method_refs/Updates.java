package oxide.lambdas_and_method_refs;

import java.util.Map;
import java.util.function.Function;
import java.util.function.Predicate;

public class Updates {
    /** True for mods whose version is not the latest one listed for their id. */
    public static Predicate<Mod> isOutdated(Map<String, String> latestById) {
        return mod -> !mod.version().equals(latestById.get(mod.id()));
    }

    /** A mod to its name, written as a lambda. */
    public static Function<Mod, String> nameByLambda() {
        return mod -> mod.name();
    }

    /** A mod to its name, written as a method reference. */
    public static Function<Mod, String> nameByMethodRef() {
        return Mod::name;
    }
}
