use std::fmt::Display;

/// The text as uppercase with a "!" at the end: banner("creeper") is "CREEPER!".
pub fn banner(text: impl Display) -> String {
    format!("{}!", text.to_string().to_uppercase())
}

/// Spawn ticks below `limit`: 0, 5, 10, ... (every fifth tick).
pub fn spawn_ticks(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|t| t % 5 == 0)
}

/// A spawn rule that adds `n` bonus damage to its input.
pub fn damage_bonus(n: u32) -> impl Fn(u32) -> u32 {
    move |d| d + n
}

/// A rule that applies `first`, then `second` to the result.
pub fn chain(first: impl Fn(u32) -> u32, second: impl Fn(u32) -> u32) -> impl Fn(u32) -> u32 {
    move |d| second(first(d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_accepts_any_display() {
        assert_eq!(banner("creeper"), "CREEPER!");
        assert_eq!(banner(42), "42!");
        assert_eq!(banner(String::from("ok")), "OK!");
    }

    #[test]
    fn spawn_ticks_yields_every_fifth_tick() {
        let v: Vec<u32> = spawn_ticks(21).collect();
        assert_eq!(v, vec![0, 5, 10, 15, 20]);
        assert_eq!(spawn_ticks(21).count(), 5);
    }

    #[test]
    fn damage_bonus_returns_closure() {
        let bonus = damage_bonus(5);
        assert_eq!(bonus(1), 6);
        assert_eq!(damage_bonus(0)(10), 10);
    }

    #[test]
    fn chain_applies_first_then_second() {
        let f = chain(damage_bonus(2), |d| d * 3);
        assert_eq!(f(4), 18, "(4 + 2) * 3");
        let g = chain(|d| d * 3, damage_bonus(2));
        assert_eq!(g(4), 14, "4 * 3 + 2");
    }
}
