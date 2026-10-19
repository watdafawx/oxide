package oxide.immutable_collections;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.Test;

class LockingTest {
    @Test
    void lockedLoaderListIsReadOnly() {
        List<String> loaders = Locking.supportedLoaders();
        assertThat(loaders).containsExactly("fabric", "forge", "neoforge", "quilt");
        assertThatThrownBy(() -> loaders.add("liteloader")).isInstanceOf(UnsupportedOperationException.class);
        assertThatThrownBy(() -> loaders.remove(0)).isInstanceOf(UnsupportedOperationException.class);
    }

    @Test
    void snapshotDetachesFromLiveList() {
        Mod sodium = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
        List<Mod> live = new ArrayList<>(List.of(sodium));
        List<Mod> snap = Locking.snapshot(live);
        live.add(new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0"));
        assertThat(snap).containsExactly(sodium);
        assertThatThrownBy(() -> snap.add(sodium)).isInstanceOf(UnsupportedOperationException.class);
    }
}
