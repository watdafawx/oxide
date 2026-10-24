package oxide.ask_for_a_list;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.Test;

class PackTest {
    @Test
    void longestModIdFromAnyList() {
        List<String> fixed = List.of("iris", "sodium", "lithium", "jei");
        assertThat(Pack.longestId(fixed)).isEqualTo("lithium");
        List<String> growing = new ArrayList<>();
        growing.add("create");
        growing.add("quark");
        assertThat(Pack.longestId(growing)).isEqualTo("create");
    }
}
