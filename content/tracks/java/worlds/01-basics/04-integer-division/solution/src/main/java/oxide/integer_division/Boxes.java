package oxide.integer_division;

public class Boxes {
    /** How many full boxes: fullBoxes(17, 5) is 3. */
    public static int fullBoxes(int items, int perBox) {
        return items / perBox;
    }

    /** Items left over: itemsLeft(17, 5) is 2. */
    public static int itemsLeft(int items, int perBox) {
        return items % perBox;
    }
}
