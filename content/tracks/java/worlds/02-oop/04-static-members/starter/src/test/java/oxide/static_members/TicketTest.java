package oxide.static_members;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class TicketTest {
    @Test
    void staticFactoryBuildsInstance() {
        Ticket ticket = Ticket.of("Fix login");
        assertThat(ticket).isNotNull();
        assertThat(ticket.title()).isEqualTo("Fix login");
    }

    @Test
    void idsAreUniquePerInstance() {
        Ticket a = Ticket.of("a");
        Ticket b = Ticket.of("b");
        Ticket c = Ticket.of("c");
        assertThat(a.id()).isPositive();
        assertThat(b.id()).isEqualTo(a.id() + 1);
        assertThat(c.id()).isEqualTo(b.id() + 1);
    }
}
