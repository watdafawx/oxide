package oxide.text_blocks_and_format;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusReportTest {
    @Test
    void buildsServerTableWithStringBuilder() {
        String table = StatusReport.table(new String[] {"survival", "creative"}, new int[] {12, 3});
        assertThat(table).isEqualTo("survival| 12\ncreative|  3\n");
        assertThat(StatusReport.table(new String[0], new int[0])).isEmpty();
    }

    @Test
    void statusJsonKeepsIndent() {
        assertThat(StatusReport.statusJson("online", 12))
                .isEqualTo("{\n  \"status\": \"online\",\n  \"players\": 12\n}\n");
    }
}
