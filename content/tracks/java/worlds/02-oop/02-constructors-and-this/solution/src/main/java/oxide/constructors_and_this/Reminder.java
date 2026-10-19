package oxide.constructors_and_this;

public class Reminder {
    private final String text;
    private final int minutes;

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
