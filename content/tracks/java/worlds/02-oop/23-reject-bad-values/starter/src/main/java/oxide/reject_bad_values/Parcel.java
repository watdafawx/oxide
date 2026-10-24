package oxide.reject_bad_values;

public record Parcel(String label, int grams) {
    // TODO: a compact constructor that throws IllegalArgumentException for negative grams,
    //       and trims the label
}
