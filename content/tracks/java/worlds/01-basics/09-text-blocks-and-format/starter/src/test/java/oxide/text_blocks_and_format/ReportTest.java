package oxide.text_blocks_and_format;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ReportTest {
    @Test
    void buildsTableWithStringBuilder() {
        String table = Report.table(new String[] {"Ada", "Bo"}, new int[] {95, 7});
        assertThat(table).isEqualToNormalizingNewlines("Ada   |  95\nBo    |   7\n");
        assertThat(Report.table(new String[0], new int[0])).isEmpty();
    }

    @Test
    void textBlockKeepsIndent() {
        assertThat(Report.jsonBlock("ok", 3)).isEqualToNormalizingNewlines("{\n  \"status\": \"ok\",\n  \"count\": 3\n}\n");
    }
}
