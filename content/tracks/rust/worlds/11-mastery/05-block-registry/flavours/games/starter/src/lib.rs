#[derive(Debug, PartialEq)]
pub struct Biome {
    pub name: &'static str,
    pub id: u16,
}

/// `biome!("plains" => 1)` is `Biome { name: "plains", id: 1 }`.
#[macro_export]
macro_rules! biome {
    // TODO: build `$crate::Biome { name: $name, id: $id }` (replace the placeholder values)
    ($name:expr => $id:expr) => {
        $crate::Biome { name: "todo", id: 0 }
    };
}

/// `biomes!["plains" => 1, "desert" => 2]` is a `Vec<Biome>` with both biomes.
#[macro_export]
macro_rules! biomes {
    // TODO: expand to `vec![ ... ]` with one `$crate::biome!($name => $id)` per pair, repeated with `$( ... ),*`
    ($($name:expr => $id:expr),* $(,)?) => {
        Vec::<$crate::Biome>::new()
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
