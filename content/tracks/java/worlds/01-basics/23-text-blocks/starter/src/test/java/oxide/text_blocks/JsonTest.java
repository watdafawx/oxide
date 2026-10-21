package oxide.text_blocks;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class JsonTest {
    @Test
    void textBlockKeepsIndent() {
        assertThat(Json.jsonBlock("ok", 3)).isEqualToNormalizingNewlines("{\n  \"status\": \"ok\",\n  \"count\": 3\n}\n");
    }
}
