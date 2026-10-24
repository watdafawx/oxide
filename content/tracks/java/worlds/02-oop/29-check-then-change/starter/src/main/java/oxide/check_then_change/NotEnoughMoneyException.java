package oxide.check_then_change;

/** Thrown when a wallet is asked for more than it holds. */
public class NotEnoughMoneyException extends RuntimeException {
    public NotEnoughMoneyException(String message) {
        super(message);
    }
}
