package oxide.each_character;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ScanTest {
    @Test
    void countsSpacesInText() {
        assertThat(Scan.countSpaces("a b c")).isEqualTo(2);
        assertThat(Scan.countSpaces("nospaces")).isEqualTo(0);
        assertThat(Scan.countSpaces("")).isEqualTo(0);
    }
}
