package oxide.a_fixed_set;

public enum TrafficLight {
    RED,
    GREEN,
    YELLOW;

    /** Only on GREEN may cars drive. */
    public boolean mayDrive() {
        throw new UnsupportedOperationException("TODO: true when this light is GREEN");
    }

    /** RED -> GREEN -> YELLOW -> RED. */
    public TrafficLight next() {
        throw new UnsupportedOperationException("TODO: a switch over this, one case per light");
    }
}
