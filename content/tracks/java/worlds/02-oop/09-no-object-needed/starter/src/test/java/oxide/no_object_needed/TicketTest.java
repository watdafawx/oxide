package oxide.no_object_needed;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.reflect.Modifier;
import org.junit.jupiter.api.Test;

class TicketTest {
    @Test
    void staticFactoryBuildsTicket() {
        Ticket ticket = Ticket.of("  Fix login ");
        assertThat(ticket).isNotNull();
        assertThat(ticket.title()).isEqualTo("Fix login");
    }

    @Test
    void constructorIsPrivate() throws Exception {
        int modifiers = Ticket.class.getDeclaredConstructor(String.class).getModifiers();
        assertThat(Modifier.isPrivate(modifiers)).as("write `private Ticket(String title)`").isTrue();
    }
}
