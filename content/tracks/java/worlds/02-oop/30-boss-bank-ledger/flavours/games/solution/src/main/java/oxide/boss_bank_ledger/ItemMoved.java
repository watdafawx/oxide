package oxide.boss_bank_ledger;

public record ItemMoved(Chest to, ItemStack stack) implements Event {}
