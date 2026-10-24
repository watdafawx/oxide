/// A mod's jar file and its size in megabytes.
#[derive(Debug, Clone, PartialEq)]
pub struct ModJar {
    pub name: String,
    pub size_mb: f64,
}

/// The sizes from smallest to largest: [20.0, 1.2, 0.5] -> [0.5, 1.2, 20.0]
pub fn sorted_sizes(sizes: Vec<f64>) -> Vec<f64> {
    let mut sizes = sizes;
    // TODO: sort the decimals, smallest first, with total_cmp
    sizes
}

/// The jar with the lowest size_mb. None for an empty list.
pub fn smallest_jar(jars: &[ModJar]) -> Option<&ModJar> {
    // TODO: pick the jar with the lowest size_mb, comparing sizes with total_cmp
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jar(name: &str, size_mb: f64) -> ModJar {
        ModJar { name: name.to_string(), size_mb }
    }

    #[test]
    fn sorted_sizes_go_small_to_large() {
        assert_eq!(sorted_sizes(vec![20.0, 1.2, 0.5]), vec![0.5, 1.2, 20.0]);
    }

    #[test]
    fn smallest_jar_picks_the_lowest_size() {
        let jars = vec![jar("create", 20.0), jar("lithium", 0.5), jar("sodium", 1.2)];
        assert_eq!(smallest_jar(&jars), Some(&jars[1]));
        assert_eq!(smallest_jar(&[]), None);
    }
}
