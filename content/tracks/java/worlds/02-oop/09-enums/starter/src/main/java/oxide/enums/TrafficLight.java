package oxide.enums;

import java.util.Locale;

public enum TrafficLight {
    RED(0), // TODO: 30 seconds
    GREEN(0), // TODO: 25 seconds
    YELLOW(0); // TODO: 5 seconds

    private final int seconds;

    TrafficLight(int seconds) {
        this.seconds = seconds;
    }

    public int seconds() {
        return seconds;
    }

    /** Finds a light by name, ignoring case and spaces. Unknown names throw IllegalArgumentException. */
    public static TrafficLight parse(String text) {
        throw new UnsupportedOperationException("TODO: clean the text, then valueOf");
    }

    /** RED -> GREEN -> YELLOW -> RED. */
    public TrafficLight next() {
        throw new UnsupportedOperationException("TODO: switch over this");
    }
}
