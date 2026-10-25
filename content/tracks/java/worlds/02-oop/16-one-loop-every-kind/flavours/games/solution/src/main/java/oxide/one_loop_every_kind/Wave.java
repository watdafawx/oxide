package oxide.one_loop_every_kind;

public class Wave {
    /** Sum of every mob's attack. Never ask which kind of mob it is. */
    public static int totalDamage(Mob[] mobs) {
        int sum = 0;
        for (Mob mob : mobs) {
            sum += mob.attackDamage();
        }
        return sum;
    }
}
