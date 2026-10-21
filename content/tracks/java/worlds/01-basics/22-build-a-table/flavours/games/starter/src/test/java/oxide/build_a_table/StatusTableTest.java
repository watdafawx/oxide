package oxide.build_a_table;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusTableTest {
    @Test
    void buildsServerTableWithStringBuilder() {
        String table = StatusTable.table(new String[] {"survival", "creative"}, new int[] {12, 3});
        assertThat(table).isEqualToNormalizingNewlines("survival| 12\ncreative|  3\n");
        assertThat(StatusTable.table(new String[0], new int[0])).isEmpty();
    }
}
