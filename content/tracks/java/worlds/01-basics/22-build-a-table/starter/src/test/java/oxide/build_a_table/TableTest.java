package oxide.build_a_table;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class TableTest {
    @Test
    void buildsTableWithStringBuilder() {
        String table = Table.table(new String[] {"Ada", "Bo"}, new int[] {95, 7});
        assertThat(table).isEqualToNormalizingNewlines("Ada   |  95\nBo    |   7\n");
        assertThat(Table.table(new String[0], new int[0])).isEmpty();
    }
}
