package oxide.a_fixed_set;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class TrafficLightTest {
    @Test
    void onlyGreenMeansDrive() {
        assertThat(TrafficLight.GREEN.mayDrive()).isTrue();
        assertThat(TrafficLight.RED.mayDrive()).isFalse();
        assertThat(TrafficLight.YELLOW.mayDrive()).isFalse();
    }

    @Test
    void nextWrapsAround() {
        assertThat(TrafficLight.RED.next()).isEqualTo(TrafficLight.GREEN);
        assertThat(TrafficLight.GREEN.next()).isEqualTo(TrafficLight.YELLOW);
        assertThat(TrafficLight.YELLOW.next()).isEqualTo(TrafficLight.RED);
    }
}
