use std::fmt::Display;

/// A chat banner: the text in uppercase with a "!" at the end.
pub fn banner(text: impl Display) -> String {
    format!("{}!", text.to_string().to_uppercase())
}

/// Ticks below `limit` on which the spawner fires: 0, 5, 10, ...
pub fn spawn_ticks(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|t| t % 5 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_accepts_any_display() {
        assert_eq!(banner("boss incoming"), "BOSS INCOMING!");
        assert_eq!(banner(3), "3!");
        assert_eq!(banner(String::from("gg")), "GG!");
    }

    #[test]
    fn spawn_ticks_yields_every_fifth_tick() {
        let v: Vec<u32> = spawn_ticks(16).collect();
        assert_eq!(v, vec![0, 5, 10, 15]);
        assert_eq!(spawn_ticks(16).count(), 4);
    }
}
