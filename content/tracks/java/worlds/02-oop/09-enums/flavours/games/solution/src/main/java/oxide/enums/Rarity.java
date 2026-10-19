package oxide.enums;

import java.util.Locale;

public enum Rarity {
    COMMON("white", 60),
    UNCOMMON("yellow", 25),
    RARE("aqua", 10),
    EPIC("light_purple", 5);

    private final String color;
    private final int dropWeight;

    Rarity(String color, int dropWeight) {
        this.color = color;
        this.dropWeight = dropWeight;
    }

    public String color() {
        return color;
    }

    public int dropWeight() {
        return dropWeight;
    }

    /** Finds a rarity by name, ignoring case and spaces. Unknown names throw IllegalArgumentException. */
    public static Rarity parse(String text) {
        return valueOf(text.trim().toUpperCase(Locale.ROOT));
    }

    /** COMMON -> UNCOMMON -> RARE -> EPIC -> COMMON. */
    public Rarity upgrade() {
        return switch (this) {
            case COMMON -> UNCOMMON;
            case UNCOMMON -> RARE;
            case RARE -> EPIC;
            case EPIC -> COMMON;
        };
    }
}
