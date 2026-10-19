#[derive(Debug, PartialEq)]
pub struct Biome {
    pub name: &'static str,
    pub id: u16,
}

/// `biome!("plains" => 1)` is `Biome { name: "plains", id: 1 }`.
#[macro_export]
macro_rules! biome {
    ($name:expr => $id:expr) => {
        $crate::Biome { name: $name, id: $id }
    };
}

/// `biomes!["plains" => 1, "desert" => 2]` is a `Vec<Biome>` with both biomes.
#[macro_export]
macro_rules! biomes {
    ($($name:expr => $id:expr),* $(,)?) => {
        vec![$( $crate::biome!($name => $id) ),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biome_macro_builds_a_biome() {
        assert_eq!(biome!("plains" => 1), Biome { name: "plains", id: 1 });
    }

    #[test]
    fn registry_macro_builds_every_biome() {
        let table = biomes!["plains" => 1, "desert" => 2,];
        assert_eq!(
            table,
            vec![Biome { name: "plains", id: 1 }, Biome { name: "desert", id: 2 }]
        );
        let none: Vec<Biome> = biomes![];
        assert!(none.is_empty());
    }
}
