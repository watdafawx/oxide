package oxide.shared_by_every_object;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class TicketTest {
    @Test
    void ticketTakesTheCounter() {
        Ticket ticket = new Ticket("Fix login");
        assertThat(ticket.title()).isEqualTo("Fix login");
        assertThat(ticket.id()).as("ids start at 1").isPositive();
    }

    @Test
    void idsAreUniquePerTicket() {
        Ticket a = new Ticket("a");
        Ticket b = new Ticket("b");
        Ticket c = new Ticket("c");
        assertThat(a.id()).isPositive();
        assertThat(b.id()).as("the counter is shared, so b comes right after a").isEqualTo(a.id() + 1);
        assertThat(c.id()).isEqualTo(b.id() + 1);
    }
}
