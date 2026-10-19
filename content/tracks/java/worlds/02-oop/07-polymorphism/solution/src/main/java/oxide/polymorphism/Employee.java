package oxide.polymorphism;

public abstract class Employee {
    private final String name;

    protected Employee(String name) {
        this.name = name;
    }

    public String name() {
        return name;
    }

    /** What this employee is paid per month, in whole dollars. */
    public abstract int monthlyPay();
}
