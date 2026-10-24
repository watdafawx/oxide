package oxide.data_in_one_line;

/** TODO: replace this whole class with a record that has the same two components: label, then grams. */
public final class Parcel {
    private final String label;
    private final int grams;

    public Parcel(String label, int grams) {
        this.label = label;
        this.grams = grams;
    }

    public String label() {
        return label;
    }

    public int grams() {
        return grams;
    }
}
