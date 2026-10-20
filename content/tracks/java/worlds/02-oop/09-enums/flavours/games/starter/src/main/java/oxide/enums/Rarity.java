package oxide.enums;

import java.util.Locale;

public enum Rarity {
    COMMON("", 0), // TODO: "white", weight 60
    UNCOMMON("", 0), // TODO: "yellow", weight 25
    RARE("", 0), // TODO: "aqua", weight 10
    EPIC("", 0); // TODO: "light_purple", weight 5

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
        throw new UnsupportedOperationException("TODO: clean the text, then valueOf");
    }

    /** COMMON -> UNCOMMON -> RARE -> EPIC -> COMMON. */
    public Rarity upgrade() {
        throw new UnsupportedOperationException("TODO: switch over this");
    }
}
