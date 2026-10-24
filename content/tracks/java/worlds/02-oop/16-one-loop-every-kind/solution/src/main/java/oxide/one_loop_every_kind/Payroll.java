package oxide.one_loop_every_kind;

public class Payroll {
    /** Sum of everyone's monthly pay. Never ask which kind of employee someone is. */
    public static int total(Employee[] staff) {
        int sum = 0;
        for (Employee employee : staff) {
            sum += employee.monthlyPay();
        }
        return sum;
    }
}
