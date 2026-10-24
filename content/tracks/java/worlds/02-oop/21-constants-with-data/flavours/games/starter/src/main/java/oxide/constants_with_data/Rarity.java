package oxide.constants_with_data;

public enum Rarity {
    COMMON("white", 0), // TODO: drop weight 60
    UNCOMMON("yellow", 0), // TODO: drop weight 25
    RARE("aqua", 0), // TODO: drop weight 10
    EPIC("light_purple", 0); // TODO: drop weight 5

    private final String color;
    private final int weight;

    Rarity(String color, int weight) {
        this.color = color;
        this.weight = weight;
    }

    public String color() {
        return color;
    }

    /** How often this tier drops, out of the total weight. */
    public int weight() {
        return weight;
    }

    /** The weights of every tier added up. */
    public static int totalWeight() {
        throw new UnsupportedOperationException("TODO: add up weight() of every tier in values()");
    }
}
