package oxide.polymorphism;

public class Hourly extends Employee {
    private final int hourlyRate;
    private final int hours;

    public Hourly(String name, int hourlyRate, int hours) {
        super(name);
        this.hourlyRate = hourlyRate;
        this.hours = hours;
    }

    @Override
    public int monthlyPay() {
        return hourlyRate * hours;
    }
}
