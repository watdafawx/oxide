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
        String[] parts = line.split(" ", 4);
        return new LogLine(parts[0], parts[1], parts[2], parts[3]);
    }

    /** Level to how many lines have it. */
    public static Map<String, Long> countsPerLevel(List<LogLine> lines) {
        return lines.stream().collect(groupingBy(LogLine::level, counting()));
    }

    /** The n sources with the most lines: highest count first, ties A-Z. */
    public static List<String> topSources(List<LogLine> lines, int n) {
        Map<String, Long> counts = lines.stream().collect(groupingBy(LogLine::source, counting()));
        Comparator<Map.Entry<String, Long>> byCount = Map.Entry.comparingByValue();
        return counts.entrySet().stream()
                .sorted(byCount.reversed().thenComparing(Map.Entry::getKey))
                .limit(n)
                .map(Map.Entry::getKey)
                .toList();
    }

    /** "HH:mm" to the number of ERROR lines in that minute, in time order. */
    public static Map<String, Long> errorsPerMinute(List<LogLine> lines) {
        return lines.stream()
                .filter(l -> l.level().equals("ERROR"))
                .collect(groupingBy(l -> l.time().substring(0, 5), TreeMap::new, counting()));
    }
}
