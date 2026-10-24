package oxide.constants_with_data;

public enum TrafficLight {
    RED(0), // TODO: red lasts 30 seconds
    GREEN(0), // TODO: green lasts 25 seconds
    YELLOW(0); // TODO: yellow lasts 5 seconds

    private final int seconds;

    TrafficLight(int seconds) {
        this.seconds = seconds;
    }

    public int seconds() {
        return seconds;
    }

    /** How long one full round of every light takes. */
    public static int fullCycle() {
        throw new UnsupportedOperationException("TODO: add up seconds() of every light in values()");
    }
}
