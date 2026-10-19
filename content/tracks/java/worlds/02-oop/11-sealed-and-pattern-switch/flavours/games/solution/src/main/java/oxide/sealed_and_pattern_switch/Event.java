package oxide.sealed_and_pattern_switch;

public sealed interface Event permits PlayerJoined, BlockBroken, ItemDropped {}
