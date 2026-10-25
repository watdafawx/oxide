package oxide.a_half_finished_parent;

public class Salaried extends Employee {
    private final int yearlyPay;

    public Salaried(String name, int yearlyPay) {
        super(name);
        this.yearlyPay = yearlyPay;
    }

    @Override
    public int monthlyPay() {
        return yearlyPay / 12;
    }
}
