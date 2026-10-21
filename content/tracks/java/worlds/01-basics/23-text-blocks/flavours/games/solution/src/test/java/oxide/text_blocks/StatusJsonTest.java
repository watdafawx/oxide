package oxide.text_blocks;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusJsonTest {
    @Test
    void statusJsonKeepsIndent() {
        assertThat(StatusJson.statusJson("online", 12))
                .isEqualToNormalizingNewlines("{\n  \"status\": \"online\",\n  \"players\": 12\n}\n");
    }
}
