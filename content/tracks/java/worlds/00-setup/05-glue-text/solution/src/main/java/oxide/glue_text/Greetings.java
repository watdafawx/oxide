package oxide.glue_text;

public class Greetings {
    /** "Hello, Ada!" */
    public static String greet(String name) {
        return "Hello, " + name + "!";
    }

    /** "Hello, Ada and Bo!" */
    public static String greetPair(String first, String second) {
        return "Hello, " + first + " and " + second + "!";
    }
}
