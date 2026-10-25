//! The macro lives in `chunk_info_macro/`. This crate uses it and tests it.
pub use chunk_info_macro::ChunkInfo;

#[cfg(test)]
mod tests {
    use super::ChunkInfo;

    #[derive(ChunkInfo)]
    #[allow(dead_code)]
    struct Chunk {
        x: i32,
        z: i32,
    }

    #[derive(ChunkInfo)]
    #[allow(dead_code)]
    enum Biome {
        Plains,
        Desert,
    }

    #[test]
    fn struct_says_name_and_field_count() {
        let c = Chunk { x: 1, z: 2 };
        assert_eq!(c.describe(), "Chunk with 2 fields");
    }

    #[test]
    fn enum_says_enum() {
        assert_eq!(Biome::Plains.describe(), "Biome (enum)");
        assert_eq!(Biome::Desert.describe(), "Biome (enum)");
    }
}
