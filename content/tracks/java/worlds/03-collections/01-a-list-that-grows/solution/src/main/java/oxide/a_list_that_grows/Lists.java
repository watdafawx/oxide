package oxide.a_list_that_grows;

import java.util.ArrayList;

public class Lists {
    /** milk, eggs, bread, in that order. */
    public static ArrayList<String> shoppingList() {
        ArrayList<String> items = new ArrayList<>();
        items.add("milk");
        items.add("eggs");
        items.add("bread");
        return items;
    }

    /** item0, item1, ... with count entries. */
    public static ArrayList<String> labels(int count) {
        ArrayList<String> items = new ArrayList<>();
        for (int i = 0; i < count; i++) {
            items.add("item" + i);
        }
        return items;
    }
}
