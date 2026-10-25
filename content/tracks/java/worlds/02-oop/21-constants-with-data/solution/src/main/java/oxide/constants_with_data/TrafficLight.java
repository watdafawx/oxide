package oxide.constants_with_data;

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

    /** How long one full round of every light takes. */
    public static int fullCycle() {
        int total = 0;
        for (TrafficLight light : values()) {
            total += light.seconds();
        }
        return total;
    }
}
