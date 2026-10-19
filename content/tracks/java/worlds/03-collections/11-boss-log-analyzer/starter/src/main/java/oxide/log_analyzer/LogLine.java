package oxide.log_analyzer;

public record LogLine(String time, String level, String source, String message) {}
