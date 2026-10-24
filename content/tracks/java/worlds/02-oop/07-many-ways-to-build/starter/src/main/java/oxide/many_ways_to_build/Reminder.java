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
        // TODO: replace these two lines with one this(...) call: the full constructor, 10 minutes
        this.text = text;
        this.minutes = 0;
    }

    public Reminder() {
        // TODO: replace these two lines with one this(...) call, with the text "Untitled"
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
