package oxide.polymorphism;

public class Salaried extends Employee {
    private final int yearlyPay;

    public Salaried(String name, int yearlyPay) {
        super(name);
        this.yearlyPay = yearlyPay;
    }

    @Override
    public int monthlyPay() {
        throw new UnsupportedOperationException("TODO: yearly pay divided by 12");
    }
}
