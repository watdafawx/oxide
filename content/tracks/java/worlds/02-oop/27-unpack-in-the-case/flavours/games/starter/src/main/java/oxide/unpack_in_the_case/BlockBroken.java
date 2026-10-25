package oxide.unpack_in_the_case;

public record BlockBroken(String player, String block, BlockPos pos) implements Event {}
