package oxide.a_fixed_set;

public enum TrafficLight {
    RED,
    GREEN,
    YELLOW;

    /** Only on GREEN may cars drive. */
    public boolean mayDrive() {
        return this == GREEN;
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
