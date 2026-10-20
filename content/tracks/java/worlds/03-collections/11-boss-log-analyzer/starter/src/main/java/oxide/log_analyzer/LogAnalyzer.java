package oxide.log_analyzer;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;

import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class LogAnalyzer {
    /** "10:15:30 ERROR db Connection refused" into time, level, source and message. */
    public static LogLine parse(String line) {
        throw new UnsupportedOperationException("TODO: cut the line into its 4 parts (the message keeps its spaces) and build a LogLine");
    }

    /** Level to how many lines have it. */
    public static Map<String, Long> countsPerLevel(List<LogLine> lines) {
        throw new UnsupportedOperationException("TODO: count the lines per level");
    }

    /** The n sources with the most lines: highest count first, ties A-Z. */
    public static List<String> topSources(List<LogLine> lines, int n) {
        throw new UnsupportedOperationException("TODO: count lines per source, sort by count (biggest first) then name, keep the first n names");
    }

    /** "HH:mm" to the number of ERROR lines in that minute, in time order. */
    public static Map<String, Long> errorsPerMinute(List<LogLine> lines) {
        throw new UnsupportedOperationException("TODO: count ERROR lines per minute (first 5 characters of the time), keys sorted");
    }
}
