package oxide.a_half_finished_parent;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.reflect.Modifier;
import org.junit.jupiter.api.Test;

class EmployeeTest {
    @Test
    void salariedPaysATwelfthOfTheYear() {
        Employee ann = new Salaried("Ann", 60000);
        assertThat(ann.monthlyPay()).isEqualTo(5000);
        assertThat(Modifier.isAbstract(Employee.class.getModifiers())).as("Employee stays abstract").isTrue();
    }

    @Test
    void hourlyPaysRateTimesHours() {
        Employee bo = new Hourly("Bo", 20, 160);
        assertThat(bo.monthlyPay()).isEqualTo(3200);
        assertThat(bo.name()).isEqualTo("Bo");
    }
}
