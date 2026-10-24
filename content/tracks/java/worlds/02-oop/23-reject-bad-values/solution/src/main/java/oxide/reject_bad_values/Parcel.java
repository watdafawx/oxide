package oxide.reject_bad_values;

public record Parcel(String label, int grams) {
    public Parcel {
        if (grams < 0) {
            throw new IllegalArgumentException("grams must not be negative: " + grams);
        }
        label = label.trim();
    }
}
