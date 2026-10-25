package oxide.one_loop_every_kind;

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
