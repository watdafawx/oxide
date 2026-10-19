package oxide.enums;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class TrafficLightTest {
    @Test
    void enumCarriesFields() {
        assertThat(TrafficLight.RED.seconds()).isEqualTo(30);
        assertThat(TrafficLight.GREEN.seconds()).isEqualTo(25);
        assertThat(TrafficLight.YELLOW.seconds()).isEqualTo(5);
    }

    @Test
    void valueOfParsesName() {
        assertThat(TrafficLight.parse("green")).isEqualTo(TrafficLight.GREEN);
        assertThat(TrafficLight.parse(" Yellow ")).isEqualTo(TrafficLight.YELLOW);
        assertThatThrownBy(() -> TrafficLight.parse("blue")).isInstanceOf(IllegalArgumentException.class);
    }

    @Test
    void nextWrapsAround() {
        assertThat(TrafficLight.RED.next()).isEqualTo(TrafficLight.GREEN);
        assertThat(TrafficLight.GREEN.next()).isEqualTo(TrafficLight.YELLOW);
        assertThat(TrafficLight.YELLOW.next()).isEqualTo(TrafficLight.RED);
    }
}
