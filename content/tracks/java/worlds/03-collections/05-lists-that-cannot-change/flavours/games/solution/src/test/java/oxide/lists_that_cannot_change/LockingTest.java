package oxide.lists_that_cannot_change;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.Test;

class LockingTest {
    @Test
    void supportedLoadersAreReadOnly() {
        List<String> loaders = Locking.supportedLoaders();
        assertThat(loaders).containsExactly("fabric", "forge", "neoforge", "quilt");
        assertThatThrownBy(() -> loaders.add("liteloader")).isInstanceOf(UnsupportedOperationException.class);
        assertThatThrownBy(() -> loaders.remove(0)).isInstanceOf(UnsupportedOperationException.class);
    }

    @Test
    void snapshotIgnoresLaterInstalls() {
        List<String> live = new ArrayList<>(List.of("sodium", "iris"));
        List<String> snap = Locking.snapshot(live);
        live.add("lithium");
        assertThat(snap).containsExactly("sodium", "iris");
        assertThatThrownBy(() -> snap.add("jei")).isInstanceOf(UnsupportedOperationException.class);
    }
}
