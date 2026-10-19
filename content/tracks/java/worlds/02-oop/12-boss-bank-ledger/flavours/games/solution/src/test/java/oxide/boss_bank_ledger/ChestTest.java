package oxide.boss_bank_ledger;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class ChestTest {
    private static ItemStack stack(String item, int count) {
        return new ItemStack(item, count);
    }

    @Test
    void placingItemsRaisesCount() {
        Chest chest = new Chest("Storage A");
        chest.apply(new ItemPlaced(stack("cobblestone", 64)));
        chest.apply(new ItemPlaced(stack("cobblestone", 10)));
        chest.apply(new ItemPlaced(stack("torch", 3)));
        assertThat(chest.count("cobblestone")).isEqualTo(74);
        assertThat(chest.count("torch")).isEqualTo(3);
        assertThat(chest.count("diamond")).isZero();
        assertThat(chest.log()).hasSize(3);
        assertThat(chest.log().get(0)).isEqualTo(new ItemPlaced(stack("cobblestone", 64)));
    }

    @Test
    void takingMoreThanStoredIsRejected() {
        Chest a = new Chest("Storage A");
        Chest b = new Chest("Storage B");
        a.apply(new ItemPlaced(stack("cobblestone", 10)));

        assertThatThrownBy(() -> a.apply(new ItemTaken(stack("cobblestone", 11))))
                .isInstanceOf(NotEnoughItemsException.class);
        assertThatThrownBy(() -> a.apply(new ItemMoved(b, stack("cobblestone", 11))))
                .isInstanceOf(NotEnoughItemsException.class);
        assertThatThrownBy(() -> a.apply(new ItemTaken(stack("diamond", 1))))
                .isInstanceOf(NotEnoughItemsException.class);

        assertThat(a.count("cobblestone")).as("a rejected event changes nothing").isEqualTo(10);
        assertThat(b.total()).isZero();
        assertThat(a.log()).as("and leaves no log entry").hasSize(1);
        assertThat(b.log()).isEmpty();

        a.apply(new ItemTaken(stack("cobblestone", 10)));
        assertThat(a.count("cobblestone")).as("taking exactly what is there is fine").isZero();
    }

    @Test
    void movingItemsChangesBothChests() {
        Chest a = new Chest("Storage A");
        Chest b = new Chest("Storage B");
        a.apply(new ItemPlaced(stack("iron_ingot", 30)));
        a.apply(new ItemMoved(b, stack("iron_ingot", 12)));
        assertThat(a.count("iron_ingot")).isEqualTo(18);
        assertThat(b.count("iron_ingot")).isEqualTo(12);
        assertThat(b.log()).containsExactly(new ItemPlaced(stack("iron_ingot", 12)));
    }

    @Test
    void reportListsEventsInOrder() {
        Chest a = new Chest("Storage A");
        Chest b = new Chest("Storage B");
        a.apply(new ItemPlaced(stack("cobblestone", 64)));
        a.apply(new ItemTaken(stack("cobblestone", 16)));
        a.apply(new ItemMoved(b, stack("cobblestone", 8)));
        assertThat(a.report()).isEqualTo(
                "1. placed 64 x cobblestone\n2. took 16 x cobblestone\n3. moved 8 x cobblestone to Storage B\nitems: 40");
        assertThat(b.report()).isEqualTo("1. placed 8 x cobblestone\nitems: 8");
        assertThat(new Chest("Empty").report()).isEqualTo("items: 0");
    }

    @Test
    void eventIsSealedToThreeKinds() {
        assertThat(Event.class.isSealed()).as("make Event `sealed ... permits ...`").isTrue();
        assertThat(Event.class.getPermittedSubclasses())
                .containsExactlyInAnyOrder(ItemPlaced.class, ItemTaken.class, ItemMoved.class);
    }
}
