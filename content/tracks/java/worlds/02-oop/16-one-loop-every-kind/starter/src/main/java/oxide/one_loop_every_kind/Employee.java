package oxide.one_loop_every_kind;

public abstract class Employee {
    private final String name;

    public Employee(String name) {
        this.name = name;
    }

    public String name() {
        return name;
    }

    /** What this employee is paid per month, in whole euros. Every kind of employee says how. */
    public abstract int monthlyPay();
}
