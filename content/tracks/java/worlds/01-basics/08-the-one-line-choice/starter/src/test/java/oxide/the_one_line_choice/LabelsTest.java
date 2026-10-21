package oxide.the_one_line_choice;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class LabelsTest {
    @Test
    void yesNoFollowsTheFlag() {
        assertThat(Labels.yesNo(true)).isEqualTo("yes");
        assertThat(Labels.yesNo(false)).isEqualTo("no");
    }

    @Test
    void pluralFollowsTheCount() {
        assertThat(Labels.plural(1)).isEqualTo("item");
        assertThat(Labels.plural(0)).isEqualTo("items");
        assertThat(Labels.plural(3)).isEqualTo("items");
    }
}
