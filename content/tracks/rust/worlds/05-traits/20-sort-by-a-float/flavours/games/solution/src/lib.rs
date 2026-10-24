#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    /// Blocks per tick.
    pub speed: f64,
}

/// The mob with the highest speed, or None when there are no mobs.
pub fn fastest(mobs: &[Mob]) -> Option<&Mob> {
    mobs.iter().max_by(|a, b| a.speed.total_cmp(&b.speed))
}

/// Sort in place, slowest first.
pub fn sort_by_speed(mobs: &mut [Mob]) {
    mobs.sort_by(|a, b| a.speed.total_cmp(&b.speed));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mob(name: &str, speed: f64) -> Mob {
        Mob { name: name.to_string(), speed }
    }

    #[test]
    fn fastest_finds_the_quickest_mob() {
        let mobs = [mob("zombie", 0.23), mob("spider", 0.3), mob("slime", 0.2)];
        assert_eq!(fastest(&mobs).map(|m| m.name.as_str()), Some("spider"));
        assert_eq!(fastest(&[]), None);
    }

    #[test]
    fn sort_by_speed_orders_mobs() {
        let mut mobs = vec![mob("spider", 0.3), mob("slime", 0.2), mob("zombie", 0.23)];
        sort_by_speed(&mut mobs);
        let names: Vec<&str> = mobs.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["slime", "zombie", "spider"]);
    }
}
