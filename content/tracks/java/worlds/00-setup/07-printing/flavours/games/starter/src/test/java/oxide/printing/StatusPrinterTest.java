package oxide.printing;

import static org.assertj.core.api.Assertions.assertThat;

import java.io.ByteArrayOutputStream;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import org.junit.jupiter.api.Test;

class StatusPrinterTest {
    @Test
    void printsStatusLine() {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        StatusPrinter.printStatus(new PrintStream(buffer, true, StandardCharsets.UTF_8), "play.example.net", 3, 20);
        assertThat(buffer.toString(StandardCharsets.UTF_8)).isEqualToNormalizingNewlines("play.example.net: 3/20 online\n");
    }

    @Test
    void formatsTpsWithTwoDecimals() {
        assertThat(StatusPrinter.formatTps(19.9766)).isEqualTo("19.98");
        assertThat(StatusPrinter.formatTps(20)).isEqualTo("20.00");
        assertThat(StatusPrinter.formatTps(7.5)).isEqualTo("7.50");
    }
}
