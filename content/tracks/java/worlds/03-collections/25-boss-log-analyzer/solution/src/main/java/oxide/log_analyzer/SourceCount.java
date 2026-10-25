package oxide.log_analyzer;

/** One source and how many lines it wrote. */
public record SourceCount(String source, long count) {}
