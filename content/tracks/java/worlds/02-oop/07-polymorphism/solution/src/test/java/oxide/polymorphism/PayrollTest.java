package oxide.polymorphism;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class PayrollTest {
    /** A kind of employee that Payroll has never heard of. */
    private static class Intern extends Employee {
        Intern(String name) {
            super(name);
        }

        @Override
        public int monthlyPay() {
            return 500;
        }
    }

    @Test
    void abstractMethodImplementedBySubtypes() {
        assertThat(new Salaried("Ann", 60000).monthlyPay()).isEqualTo(5000);
        assertThat(new Hourly("Bo", 20, 160).monthlyPay()).isEqualTo(3200);
    }

    @Test
    void payrollSumsAllSubtypes() {
        List<Employee> staff = List.of(new Salaried("Ann", 60000), new Hourly("Bo", 20, 160), new Intern("Cy"));
        assertThat(Payroll.total(staff)).isEqualTo(5000 + 3200 + 500);
        assertThat(Payroll.total(List.of())).isZero();
    }
}
