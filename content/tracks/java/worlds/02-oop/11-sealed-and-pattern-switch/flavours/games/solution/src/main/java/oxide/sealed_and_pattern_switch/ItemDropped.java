package oxide.sealed_and_pattern_switch;

public record ItemDropped(String item, int count) implements Event {}
