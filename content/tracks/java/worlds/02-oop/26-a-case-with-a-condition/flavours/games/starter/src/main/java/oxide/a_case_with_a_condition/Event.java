package oxide.a_case_with_a_condition;

public sealed interface Event permits PlayerJoined, BlockBroken, ItemDropped {}
