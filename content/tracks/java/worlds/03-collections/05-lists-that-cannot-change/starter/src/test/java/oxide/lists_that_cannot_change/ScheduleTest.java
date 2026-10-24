package oxide.lists_that_cannot_change;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.Test;

class ScheduleTest {
    @Test
    void openingDaysAreReadOnly() {
        List<String> days = Schedule.openingDays();
        assertThat(days).containsExactly("Mon", "Tue", "Wed");
        assertThatThrownBy(() -> days.add("Thu")).isInstanceOf(UnsupportedOperationException.class);
        assertThatThrownBy(() -> days.remove(0)).isInstanceOf(UnsupportedOperationException.class);
    }

    @Test
    void snapshotIgnoresLaterChanges() {
        List<String> source = new ArrayList<>(List.of("a", "b"));
        List<String> snap = Schedule.snapshot(source);
        source.add("c");
        assertThat(snap).containsExactly("a", "b");
        assertThatThrownBy(() -> snap.add("x")).isInstanceOf(UnsupportedOperationException.class);
    }
}
