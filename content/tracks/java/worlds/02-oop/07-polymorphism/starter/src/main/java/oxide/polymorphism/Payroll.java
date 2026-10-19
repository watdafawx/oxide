package oxide.polymorphism;

import java.util.List;

public class Payroll {
    /** Sum of everyone's monthly pay. Never ask which subtype an employee is. */
    public static int total(List<Employee> staff) {
        throw new UnsupportedOperationException("TODO: loop over staff and add up monthlyPay()");
    }
}
