package oxide.polymorphism;

import java.util.List;

public class Payroll {
    /** Sum of everyone's monthly pay. Never ask which subtype an employee is. */
    public static int total(List<Employee> staff) {
        int sum = 0;
        for (Employee employee : staff) {
            sum += employee.monthlyPay();
        }
        return sum;
    }
}
