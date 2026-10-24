package oxide.many_ways_to_build;

public class Reminder {
    private final String text;
    private final int minutes;

    /** The full constructor: the rules live here. */
    public Reminder(String text, int minutes) {
        this.text = text.trim();
        this.minutes = Math.max(1, minutes);
    }

    public Reminder(String text) {
        this(text, 10);
    }

    public Reminder() {
        this("Untitled");
    }

    public String text() {
        return text;
    }

    public int minutes() {
        return minutes;
    }
}
