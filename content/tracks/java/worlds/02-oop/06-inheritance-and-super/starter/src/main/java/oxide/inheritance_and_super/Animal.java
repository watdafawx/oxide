package oxide.inheritance_and_super;

public class Animal {
    private final String name;

    public Animal(String name) {
        this.name = name;
    }

    public String name() {
        return name;
    }

    public String sound() {
        return "...";
    }

    public String describe() {
        return name + " says " + sound();
    }
}
