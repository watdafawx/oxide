package oxide.boss_bank_ledger;

public class NotEnoughItemsException extends RuntimeException {
    public NotEnoughItemsException(String message) {
        super(message);
    }
}
