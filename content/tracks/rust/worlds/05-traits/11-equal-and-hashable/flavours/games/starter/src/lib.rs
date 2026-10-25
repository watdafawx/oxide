#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

// TODO: when Rust asks for more traits, add them to this derive line
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

/// How many different chunks the player walked into.
pub fn loaded_chunks(visits: &[ChunkPos]) -> usize {
    // TODO: collect the positions into a HashSet<ChunkPos> and return how many it holds
    visits.len()
}

/// How many times each chunk was visited.
pub fn visit_counts(visits: &[ChunkPos]) -> HashMap<ChunkPos, u32> {
    // TODO: for each chunk, add one to its count with counts.entry(..).or_insert(0)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk(x: i32, z: i32) -> ChunkPos {
        ChunkPos { x, z }
    }

    #[test]
    fn loaded_chunks_counts_each_chunk_once() {
        let walk = [chunk(0, 0), chunk(1, 0), chunk(0, 0), chunk(1, 0), chunk(-2, 3)];
        assert_eq!(loaded_chunks(&walk), 3);
        assert_eq!(loaded_chunks(&[]), 0);
    }

    #[test]
    fn visit_counts_tallies_each_chunk() {
        let counts: Vec<(ChunkPos, u32)> = visit_counts(&[chunk(0, 0), chunk(1, 0), chunk(0, 0)]).into_iter().collect();
        assert_eq!(counts.len(), 2, "two different chunks were visited");
        assert!(counts.contains(&(chunk(0, 0), 2)), "(0, 0) was visited twice");
        assert!(counts.contains(&(chunk(1, 0), 1)), "(1, 0) was visited once");
    }
}
