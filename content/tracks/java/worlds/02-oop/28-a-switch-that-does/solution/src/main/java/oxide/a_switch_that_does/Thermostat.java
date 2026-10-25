package oxide.a_switch_that_does;

public class Thermostat {
    private int target = 20;
    private int resets;

    public int target() {
        return target;
    }

    /** How many times the thermostat was reset. */
    public int resets() {
        return resets;
    }

    /** Changes the target: Warmer raises it, Cooler lowers it, Reset sets it to 20 and counts the reset. */
    public void apply(Adjust adjust) {
        switch (adjust) {
            case Warmer w -> target += w.degrees();
            case Cooler c -> target -= c.degrees();
            case Reset r -> {
                target = 20;
                resets++;
            }
        }
    }
}
