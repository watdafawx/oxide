package oxide.enums;

import java.util.Locale;

public enum TrafficLight {
    RED(30),
    GREEN(25),
    YELLOW(5);

    private final int seconds;

    TrafficLight(int seconds) {
        this.seconds = seconds;
    }

    public int seconds() {
        return seconds;
    }

    /** Finds a light by name, ignoring case and spaces. Unknown names throw IllegalArgumentException. */
    public static TrafficLight parse(String text) {
        return valueOf(text.trim().toUpperCase(Locale.ROOT));
    }

    /** RED -> GREEN -> YELLOW -> RED. */
    public TrafficLight next() {
        return switch (this) {
            case RED -> GREEN;
            case GREEN -> YELLOW;
            case YELLOW -> RED;
        };
    }
}
