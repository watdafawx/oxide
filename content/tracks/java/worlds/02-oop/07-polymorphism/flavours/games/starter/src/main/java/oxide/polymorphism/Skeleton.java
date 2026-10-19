package oxide.polymorphism;

public class Skeleton extends Mob {
    private final int arrows;

    public Skeleton(String name, int arrows) {
        super(name);
        this.arrows = arrows;
    }

    @Override
    public int attackDamage() {
        throw new UnsupportedOperationException("TODO: 2 per arrow");
    }
}
