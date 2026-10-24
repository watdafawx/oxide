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

    /// Generic: a chunk section holds blocks, light and a palette of any cloneable type.
    #[derive(ChunkInfo)]
    #[allow(dead_code)]
    struct Section<T: Clone> {
        blocks: T,
        light: T,
        palette: T,
    }

    #[test]
    fn generic_struct_gets_a_correct_impl() {
        let s = Section { blocks: 1u8, light: 2, palette: 3 };
        assert_eq!(s.describe(), "Section with 3 fields");
        let names = Section { blocks: "stone", light: "15", palette: "overworld" };
        assert_eq!(names.describe(), "Section with 3 fields");
    }

    #[test]
    fn plain_struct_still_works() {
        assert_eq!(Chunk { x: 1, z: 2 }.describe(), "Chunk with 2 fields");
    }
}
