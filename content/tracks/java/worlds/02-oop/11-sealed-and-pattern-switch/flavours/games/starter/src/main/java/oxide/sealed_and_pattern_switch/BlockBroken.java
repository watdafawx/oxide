package oxide.sealed_and_pattern_switch;

public record BlockBroken(String player, String block, BlockPos pos) implements Event {}
