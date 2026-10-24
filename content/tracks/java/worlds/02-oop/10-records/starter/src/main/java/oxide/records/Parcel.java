package oxide.records;

/** TODO: turn this class into a record with the same two components (label, then grams). */
public final class Parcel {
    private final String label;
    private final int grams;

    public Parcel(String label, int grams) {
        // TODO: in the record version this becomes a compact constructor that rejects negative grams
        this.label = label;
        this.grams = grams;
    }

    public String label() {
        return label;
    }

    public int grams() {
        return grams;
    }

    public Parcel withGrams(int newGrams) {
        throw new UnsupportedOperationException("TODO: return a copy with the new weight");
    }
}
