package oxide.log_analyzer;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class LogAnalyzer {
    /** "10:15:30 ERROR db Connection refused" into time, level, source and message. */
    public static LogLine parse(String line) {
        // TODO: split into at most 4 parts (the message keeps its spaces) and build a LogLine
        return null;
    }

    /** Level to how many lines have it. */
    public static Map<String, Long> countsPerLevel(List<LogLine> lines) {
        // TODO: group the lines by level and count each group
        return Map.of();
    }

    /** The n sources with the most lines: highest count first, ties A to Z. */
    public static List<String> topSources(List<LogLine> lines, int n) {
        // TODO: count lines per source, turn each entry into a SourceCount in a list,
        // sort it by count (highest first) then source, keep the first n source names
        return List.of();
    }

    /** "HH:mm" to the number of ERROR lines in that minute, in time order. */
    public static Map<String, Long> errorsPerMinute(List<LogLine> lines) {
        // TODO: keep ERROR lines, count them per minute (time.substring(0, 5) is "HH:mm"),
        // and return the counts copied into a new TreeMap so the minutes are sorted
        return Map.of();
    }
}
