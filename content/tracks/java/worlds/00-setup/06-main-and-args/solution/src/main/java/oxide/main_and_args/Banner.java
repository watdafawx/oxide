package oxide.main_and_args;

public class Banner {
    public static void main(String[] args) {
        System.out.println(banner(args));
    }

    /** "Hello, <first word>!" */
    public static String banner(String[] args) {
        return "Hello, " + args[0] + "!";
    }

    /** "Hello, <first word> and <second word>!" */
    public static String pairBanner(String[] args) {
        return "Hello, " + args[0] + " and " + args[1] + "!";
    }
}
