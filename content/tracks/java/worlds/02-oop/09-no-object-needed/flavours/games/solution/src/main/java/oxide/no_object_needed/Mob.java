package oxide.no_object_needed;

public class Mob {
    private final String kind;

    private Mob(String kind) {
        this.kind = kind;
    }

    /** Spawns a mob with a clean, lower-case kind. */
    public static Mob spawn(String kind) {
        return new Mob(kind.trim().toLowerCase());
    }

    public String kind() {
        return kind;
    }
}
