package oxide.a_switch_that_does;

public sealed interface Command permits Heal, Hurt, Respawn {}
