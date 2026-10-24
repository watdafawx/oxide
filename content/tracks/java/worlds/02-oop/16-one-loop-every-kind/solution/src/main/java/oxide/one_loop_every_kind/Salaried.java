package oxide.one_loop_every_kind;

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
