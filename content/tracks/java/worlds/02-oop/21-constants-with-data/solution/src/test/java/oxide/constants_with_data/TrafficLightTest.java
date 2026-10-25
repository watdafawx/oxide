package oxide.constants_with_data;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class TrafficLightTest {
    @Test
    void enumCarriesSeconds() {
        assertThat(TrafficLight.RED.seconds()).isEqualTo(30);
        assertThat(TrafficLight.GREEN.seconds()).isEqualTo(25);
        assertThat(TrafficLight.YELLOW.seconds()).isEqualTo(5);
    }

    @Test
    void fullCycleAddsEveryLight() {
        assertThat(TrafficLight.fullCycle()).isEqualTo(60);
    }
}
