package oxide.polymorphism;

import java.util.List;

public class Wave {
    /** Total damage if every mob attacks once. Never ask which subtype a mob is. */
    public static int totalDamage(List<Mob> mobs) {
        int sum = 0;
        for (Mob mob : mobs) {
            sum += mob.attackDamage();
        }
        return sum;
    }
}
