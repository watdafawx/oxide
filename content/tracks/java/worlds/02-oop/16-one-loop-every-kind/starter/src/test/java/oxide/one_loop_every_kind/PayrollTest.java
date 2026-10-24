package oxide.one_loop_every_kind;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PayrollTest {
    /** A kind of employee that Payroll has never heard of. */
    static class Intern extends Employee {
        Intern(String name) {
            super(name);
        }

        @Override
        public int monthlyPay() {
            return 500;
        }
    }

    @Test
    void payrollSumsAMixedTeam() {
        Employee[] staff = {new Salaried("Ann", 60000), new Hourly("Bo", 20, 160)};
        assertThat(Payroll.total(staff)).isEqualTo(5000 + 3200);
        assertThat(Payroll.total(new Employee[0])).as("nobody, nothing to pay").isZero();
    }

    @Test
    void payrollPaysAKindItHasNeverSeen() {
        Employee[] staff = {new Salaried("Ann", 60000), new Intern("Cy"), new Intern("Di")};
        assertThat(Payroll.total(staff)).isEqualTo(5000 + 500 + 500);
    }
}
