package oxide.a_switch_that_does;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ThermostatTest {
    @Test
    void warmerAndCoolerMoveTheTarget() {
        Thermostat hall = new Thermostat();
        hall.apply(new Warmer(3));
        assertThat(hall.target()).isEqualTo(23);
        hall.apply(new Cooler(5));
        assertThat(hall.target()).isEqualTo(18);
    }

    @Test
    void resetGoesBackTo20AndIsCounted() {
        Thermostat hall = new Thermostat();
        hall.apply(new Warmer(4));
        hall.apply(new Reset());
        assertThat(hall.target()).isEqualTo(20);
        hall.apply(new Reset());
        assertThat(hall.resets()).isEqualTo(2);
    }
}
