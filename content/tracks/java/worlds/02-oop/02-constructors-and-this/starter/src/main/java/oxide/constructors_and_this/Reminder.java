package oxide.constructors_and_this;

public class Reminder {
    private final String text;
    private final int minutes;

    public Reminder(String text, int minutes) {
        // TODO: trim the text and keep minutes at 1 or more
        this.text = text;
        this.minutes = minutes;
    }

    public Reminder(String text) {
        // TODO: call the two-argument constructor with this(...), 10 minutes (delete the two lines below it: this(...) must come first)
        this.text = text;
        this.minutes = 0;
    }

    public Reminder() {
        // TODO: call another constructor with the text "Untitled" (delete the two lines below it)
        this.text = "";
        this.minutes = 0;
    }

    public String text() {
        return text;
    }

    public int minutes() {
        return minutes;
    }
}
