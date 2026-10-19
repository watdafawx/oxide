package oxide.main_and_args;

public class StatusCommand {
    public static void main(String[] args) {
        System.out.println(header(args));
    }

    /** "Status of <first arg>", or "Status of localhost" when there are no args. */
    public static String header(String[] args) {
        return null; // TODO: use args[0] as the server address when there is one, otherwise "localhost"
    }
}
