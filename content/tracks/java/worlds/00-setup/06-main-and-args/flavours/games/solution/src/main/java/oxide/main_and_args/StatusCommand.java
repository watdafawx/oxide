package oxide.main_and_args;

public class StatusCommand {
    public static void main(String[] args) {
        System.out.println(header(args));
    }

    /** "Status of <first arg>", or "Status of localhost" when there are no args. */
    public static String header(String[] args) {
        if (args.length == 0) {
            return "Status of localhost";
        }
        return "Status of " + args[0];
    }
}
