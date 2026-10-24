package oxide.check_then_change;

/** Thrown when a player is asked for more XP levels than they have. */
public class NotEnoughLevelsException extends RuntimeException {
    public NotEnoughLevelsException(String message) {
        super(message);
    }
}
