package oxide.a_fixed_set;

public enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    EPIC;

    /** Only COMMON items stack in the junk chest. */
    public boolean isCommon() {
        return this == COMMON;
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
