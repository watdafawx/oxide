package oxide.records;

public record Parcel(String label, int grams) {
    public Parcel {
        if (grams < 0) {
            throw new IllegalArgumentException("grams must not be negative: " + grams);
        }
    }

    public Parcel withGrams(int newGrams) {
        return new Parcel(label, newGrams);
    }
}
