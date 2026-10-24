#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

/// The chunks from `x = first_x` to `x = last_x` (both included) in row `z`.
pub struct ChunkRow {
    x: i32,
    last_x: i32,
    z: i32,
}

impl ChunkRow {
    pub fn new(first_x: i32, last_x: i32, z: i32) -> Self {
        ChunkRow { x: first_x, last_x, z }
    }
}

impl Iterator for ChunkRow {
    type Item = ChunkPos;

    fn next(&mut self) -> Option<ChunkPos> {
        // TODO: None once x is past last_x; otherwise hand back the chunk at x and step x one east
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_walks_the_row_then_stops() {
        let mut row = ChunkRow::new(0, 2, 5);
        assert_eq!(row.next(), Some(ChunkPos { x: 0, z: 5 }));
        assert_eq!(row.next(), Some(ChunkPos { x: 1, z: 5 }));
        assert_eq!(row.next(), Some(ChunkPos { x: 2, z: 5 }));
        assert_eq!(row.next(), None, "last_x is included, then the row ends");
    }

    #[test]
    fn row_collects_like_a_range() {
        let v: Vec<ChunkPos> = ChunkRow::new(-1, 1, 0).collect();
        assert_eq!(v, vec![ChunkPos { x: -1, z: 0 }, ChunkPos { x: 0, z: 0 }, ChunkPos { x: 1, z: 0 }]);
    }
}
