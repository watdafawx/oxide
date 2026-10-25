package oxide.no_object_needed;

public class Mob {
    private final String kind;

    // TODO: make this constructor private, so spawn(...) is the only way in
    public Mob(String kind) {
        this.kind = kind;
    }

    /** Spawns a mob with a clean, lower-case kind. */
    public static Mob spawn(String kind) {
        throw new UnsupportedOperationException("TODO: build a new Mob from the trimmed, lower-case kind and hand it back");
    }

    public String kind() {
        return kind;
    }
}
