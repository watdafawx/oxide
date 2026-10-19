use rayon::prelude::*;

/// One jar from the mods folder.
#[derive(Debug, Clone)]
pub struct ModJar {
    pub name: String,
    pub bytes: u64,
}

impl ModJar {
    pub fn new(name: &str, bytes: u64) -> ModJar {
        ModJar {
            name: name.to_string(),
            bytes,
        }
    }
}

/// Total size of all jars.
pub fn total_bytes(jars: &[ModJar]) -> u64 {
    // TODO: par_iter + map + sum
    todo!()
}

/// The length of every jar's name, in the same order.
pub fn name_lengths(jars: &[ModJar]) -> Vec<usize> {
    // TODO: par_iter + map + collect
    todo!()
}

/// Given: a mod id is lucky when it is prime (trial division).
fn is_lucky(id: u32) -> bool {
    id >= 2 && (2..).take_while(|d| d * d <= id).all(|d| !id.is_multiple_of(d))
}

/// How many mod ids below `limit` are lucky?
pub fn count_lucky_ids(limit: u32) -> usize {
    // TODO: (0..limit).into_par_iter() + filter + count
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_bytes_runs_in_parallel() {
        let jars = vec![
            ModJar::new("sodium.jar", 100),
            ModJar::new("iris.jar", 200),
            ModJar::new("jei.jar", 300),
        ];
        assert_eq!(total_bytes(&jars), 600);
        let big: Vec<ModJar> = (1..=1000).map(|n| ModJar::new("m.jar", n)).collect();
        assert_eq!(total_bytes(&big), 500_500);
        assert_eq!(total_bytes(&[]), 0);
    }

    #[test]
    fn name_lengths_keep_folder_order() {
        let jars = vec![
            ModJar::new("a", 1),
            ModJar::new("abc", 1),
            ModJar::new("", 1),
            ModJar::new("hello", 1),
        ];
        assert_eq!(name_lengths(&jars), vec![1, 3, 0, 5]);
    }

    #[test]
    fn count_lucky_ids_uses_into_par_iter() {
        assert_eq!(count_lucky_ids(0), 0);
        assert_eq!(count_lucky_ids(10), 4);
        assert_eq!(count_lucky_ids(100), 25);
        assert_eq!(count_lucky_ids(10_000), 1229);
    }
}
