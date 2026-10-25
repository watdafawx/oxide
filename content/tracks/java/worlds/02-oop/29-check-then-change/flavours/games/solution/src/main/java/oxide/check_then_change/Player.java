package oxide.check_then_change;

public class Player {
    private final String name;
    private int levels;

    public Player(String name, int levels) {
        this.name = name;
        this.levels = levels;
    }

    public int levels() {
        return levels;
    }

    /** Spends XP levels on an enchantment. Asking for more than the player has throws and changes nothing. */
    public void spendLevels(int amount) {
        if (amount > levels) {
            throw new NotEnoughLevelsException(name + " has " + levels + " levels, not " + amount);
        }
        levels -= amount;
    }

    /** Gives XP levels to another player. A refused gift changes neither player. */
    public void giveLevels(Player to, int amount) {
        spendLevels(amount);
        to.levels += amount;
    }
}
