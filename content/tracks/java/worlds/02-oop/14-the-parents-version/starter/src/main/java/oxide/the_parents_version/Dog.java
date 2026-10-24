package oxide.the_parents_version;

public class Dog extends Animal {
    public Dog(String name) {
        // TODO: hand this dog's own name to Animal's constructor, not "Dog"
        super("Dog");
    }

    @Override
    public String sound() {
        return "Woof";
    }

    // TODO: override describe(): Animal's own describe() text, plus " and wags its tail"
}
