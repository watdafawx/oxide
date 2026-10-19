package oxide.constructors_and_this;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ReminderTest {
    @Test
    void defaultConstructorSetsDefaults() {
        Reminder reminder = new Reminder();
        assertThat(reminder.text()).isEqualTo("Untitled");
        assertThat(reminder.minutes()).isEqualTo(10);
    }

    @Test
    void chainedConstructorReusesFirst() {
        Reminder reminder = new Reminder("  Call mum  ");
        assertThat(reminder.text()).as("text is trimmed by the full constructor").isEqualTo("Call mum");
        assertThat(reminder.minutes()).isEqualTo(10);
        assertThat(new Reminder("x", 0).minutes()).as("at least one minute").isEqualTo(1);
    }
}
