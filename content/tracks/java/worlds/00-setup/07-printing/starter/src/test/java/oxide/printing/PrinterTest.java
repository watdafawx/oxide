package oxide.printing;

import static org.assertj.core.api.Assertions.assertThat;

import java.io.ByteArrayOutputStream;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import org.junit.jupiter.api.Test;

class PrinterTest {
    @Test
    void printsGreetingLine() {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        Printer.printGreeting(new PrintStream(buffer, true, StandardCharsets.UTF_8), "Ada");
        assertThat(buffer.toString(StandardCharsets.UTF_8)).isEqualToNormalizingNewlines("Hello, Ada!\n");
    }

    @Test
    void formatsPriceWithTwoDecimals() {
        assertThat(Printer.formatPrice(12.5)).isEqualTo("$12.50");
        assertThat(Printer.formatPrice(3)).isEqualTo("$3.00");
        assertThat(Printer.formatPrice(0.999)).isEqualTo("$1.00");
    }
}
