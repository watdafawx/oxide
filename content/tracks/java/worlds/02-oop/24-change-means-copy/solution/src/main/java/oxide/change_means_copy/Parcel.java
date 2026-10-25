package oxide.change_means_copy;

public record Parcel(String label, int grams) {
    public Parcel {
        if (grams < 0) {
            throw new IllegalArgumentException("grams must not be negative: " + grams);
        }
        label = label.trim();
    }

    /** A copy of this parcel with a new weight. This one never changes. */
    public Parcel withGrams(int newGrams) {
        return new Parcel(label, newGrams);
    }
}
