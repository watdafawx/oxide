#[derive(Debug, PartialEq, Default)]
pub struct Drops {
    pub xp: u32,
    pub items: Vec<String>,
}

/// A brand-new mob type drops nothing yet.
pub fn no_drops() -> Drops {
    Drops::default()
}

#[derive(Debug, PartialEq)]
pub struct MobStats {
    pub health: u32,
    pub armor: u32,
}

impl Default for MobStats {
    fn default() -> Self {
        MobStats { health: 20, armor: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_default_to_nothing() {
        assert_eq!(no_drops(), Drops { xp: 0, items: Vec::new() });
    }

    #[test]
    fn mob_stats_default_to_twenty_health() {
        let stats = MobStats::default();
        assert_eq!(stats.health, 20);
        assert_eq!(stats.armor, 0);
        let from_config: Vec<MobStats> = Vec::new();
        let configured: Option<MobStats> = from_config.into_iter().next(); // None: no config line
        assert_eq!(configured.unwrap_or_default(), stats, "generic code uses your Default too");
    }
}
