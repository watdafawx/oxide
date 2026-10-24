package oxide.the_parents_version;

public class Dog extends Animal {
    public Dog(String name) {
        super(name);
    }

    @Override
    public String sound() {
        return "Woof";
    }

    @Override
    public String describe() {
        return super.describe() + " and wags its tail";
    }
}
