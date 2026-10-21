package oxide.main_and_args;

public class StatusCommand {
    public static void main(String[] args) {
        System.out.println(header(args));
    }

    /** "Status of <first word>". */
    public static String header(String[] args) {
        return "Status of " + args[0];
    }

    /** "Status of <first word> and <second word>". */
    public static String compareHeader(String[] args) {
        return "Status of " + args[0] + " and " + args[1];
    }
}
