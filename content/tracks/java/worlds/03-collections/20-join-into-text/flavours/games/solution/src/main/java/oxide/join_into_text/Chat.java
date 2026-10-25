package oxide.join_into_text;

import static java.util.stream.Collectors.joining;

import java.util.List;

public class Chat {
    /** "[Sodium, Iris Shaders]", or "[]" for no mods. */
    public static String packLine(List<Mod> mods) {
        return mods.stream().map(Mod::name).collect(joining(", ", "[", "]"));
    }
}
