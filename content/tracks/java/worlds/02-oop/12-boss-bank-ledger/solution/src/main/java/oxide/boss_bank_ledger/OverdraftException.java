package oxide.boss_bank_ledger;

public class OverdraftException extends RuntimeException {
    public OverdraftException(String message) {
        super(message);
    }
}
