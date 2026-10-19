package oxide.boss_bank_ledger;

public record ItemStack(String item, int count) {
    public ItemStack {
        if (count <= 0) {
            throw new IllegalArgumentException("a stack holds at least one item: " + count);
        }
    }
}
