package oxide.is_a;

public class Dog extends Animal {
    public Dog(String name) {
        super(name);
    }

    @Override
    public String sound() {
        return "Woof";
    }
}
