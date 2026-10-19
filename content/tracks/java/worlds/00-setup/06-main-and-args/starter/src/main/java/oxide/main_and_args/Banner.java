package oxide.main_and_args;

public class Banner {
    public static void main(String[] args) {
        System.out.println(banner(args));
    }

    /** "Hello, <first arg>!", or "Hello, Oxide!" when there are no args. */
    public static String banner(String[] args) {
        return null; // TODO: use args[0] when there is one, otherwise "Oxide"
    }
}
