use std::collections::HashMap;

/// [bone, flesh, bone, bone] -> bone: 3, flesh: 1
pub fn count_drops(log: &[&str]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for item in log {
        *counts.entry(item.to_string()).or_insert(0) += 1;
    }
    counts
}

/// Put `amount` of `item` in the chest. Known items go up; a new item starts at 0 first.
pub fn add_to_chest(chest: &mut HashMap<String, u32>, item: &str, amount: u32) {
    *chest.entry(item.to_string()).or_insert(0) += amount;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_drops_tallies_each_item() {
        let counts = count_drops(&["bone", "flesh", "bone", "bone"]);
        assert_eq!(counts.get("bone"), Some(&3));
        assert_eq!(counts.get("flesh"), Some(&1));
        assert_eq!(counts.len(), 2);
        assert!(count_drops(&[]).is_empty(), "no kills, no drops");
    }

    #[test]
    fn add_to_chest_raises_or_creates_the_count() {
        let mut chest = HashMap::new();
        chest.insert("coal".to_string(), 12);
        add_to_chest(&mut chest, "coal", 30);
        add_to_chest(&mut chest, "diamond", 2);
        assert_eq!(chest.get("coal"), Some(&42), "12 + 30");
        assert_eq!(chest.get("diamond"), Some(&2), "a new item starts at 0");
    }
}
