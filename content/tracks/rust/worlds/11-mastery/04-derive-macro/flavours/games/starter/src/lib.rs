//! The macro lives in `chunk_info_macro/`. This crate re-exports it and tests it.
pub use derive_hello::ChunkInfo;

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

    #[derive(ChunkInfo)]
    #[allow(dead_code)]
    struct Section<T: Clone> {
        blocks: T,
        light: T,
        palette: T,
    }

    #[test]
    fn struct_reports_name_and_field_count() {
        let c = Chunk { x: 1, z: 2 };
        assert_eq!(c.describe(), "Chunk with 2 fields");
    }

    #[test]
    fn enum_says_enum() {
        assert_eq!(Biome::Plains.describe(), "Biome (enum)");
        assert_eq!(Biome::Desert.describe(), "Biome (enum)");
    }

    #[test]
    fn generic_struct_gets_a_correct_impl() {
        let s = Section { blocks: 1u8, light: 2, palette: 3 };
        assert_eq!(s.describe(), "Section with 3 fields");
    }
}
