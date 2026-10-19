package oxide.lambdas_and_method_refs;

import java.util.Map;
import java.util.function.Function;
import java.util.function.Predicate;

public class Updates {
    /** True for mods whose version is not the latest one listed for their id. */
    public static Predicate<Mod> isOutdated(Map<String, String> latestById) {
        throw new UnsupportedOperationException("TODO: return a lambda mod -> ... comparing mod.version() with latestById.get(mod.id())");
    }

    /** A mod to its name, written as a lambda. */
    public static Function<Mod, String> nameByLambda() {
        throw new UnsupportedOperationException("TODO: return a lambda mod -> ...");
    }

    /** A mod to its name, written as a method reference. */
    public static Function<Mod, String> nameByMethodRef() {
        throw new UnsupportedOperationException("TODO: return a method reference to Mod's name accessor");
    }
}
