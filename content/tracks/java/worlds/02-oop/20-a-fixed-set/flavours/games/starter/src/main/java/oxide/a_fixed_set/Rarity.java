package oxide.a_fixed_set;

public enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    EPIC;

    /** Only COMMON items stack in the junk chest. */
    public boolean isCommon() {
        throw new UnsupportedOperationException("TODO: true when this rarity is COMMON");
    }

    /** COMMON -> UNCOMMON -> RARE -> EPIC -> COMMON. */
    public Rarity upgrade() {
        throw new UnsupportedOperationException("TODO: a switch over this, one case per rarity");
    }
}
