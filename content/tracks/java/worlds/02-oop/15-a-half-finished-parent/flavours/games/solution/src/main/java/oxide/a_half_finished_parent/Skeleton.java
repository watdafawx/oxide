package oxide.a_half_finished_parent;

public class Skeleton extends Mob {
    private final int arrows;

    public Skeleton(int arrows) {
        super("Skeleton");
        this.arrows = arrows;
    }

    @Override
    public int attackDamage() {
        return arrows * 2;
    }
}
