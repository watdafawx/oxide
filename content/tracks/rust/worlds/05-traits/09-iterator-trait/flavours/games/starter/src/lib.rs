#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

/// Every chunk within `radius` of the origin, as a square: x from -radius to radius (outer loop),
/// z from -radius to radius (inner loop).
pub struct ChunksInRadius {
    radius: i32,
    x: i32,
    z: i32,
}

impl ChunksInRadius {
    pub fn new(radius: i32) -> Self {
        ChunksInRadius { radius, x: -radius, z: -radius }
    }
}

impl Iterator for ChunksInRadius {
    type Item = ChunkPos;

    fn next(&mut self) -> Option<ChunkPos> {
        // TODO: return None once x is past the radius. Otherwise return the current chunk,
        // then step z forward; when z passes the radius, wrap it to -radius and step x.
        todo!()
    }
}

/// Chunks in the radius where x + z is even (that is where the ore veins are).
pub fn ore_chunks(radius: i32) -> Vec<ChunkPos> {
    // TODO: keep the chunks where x + z is even (careful: negatives, see the lesson) and collect
    Vec::new()
}

/// Squared distances x*x + z*z of every chunk in the radius, added up.
pub fn total_distance_sq(radius: i32) -> i32 {
    // TODO: map each chunk to x*x + z*z and sum
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(x: i32, z: i32) -> ChunkPos {
        ChunkPos { x, z }
    }

    #[test]
    fn chunks_in_radius_yields_nine_then_stops() {
        let v: Vec<ChunkPos> = ChunksInRadius::new(1).collect();
        assert_eq!(v.len(), 9);
        assert_eq!(v[0], c(-1, -1));
        assert_eq!(v[1], c(-1, 0), "z moves first");
        assert_eq!(v[8], c(1, 1));
        let origin_only: Vec<ChunkPos> = ChunksInRadius::new(0).collect();
        assert_eq!(origin_only, vec![c(0, 0)], "radius 0 is just the origin chunk");
    }

    #[test]
    fn ore_chunks_keep_even_sums() {
        assert_eq!(
            ore_chunks(1),
            vec![c(-1, -1), c(-1, 1), c(0, 0), c(1, -1), c(1, 1)]
        );
        assert_eq!(ore_chunks(0), vec![c(0, 0)]);
    }

    #[test]
    fn total_distance_sq_is_twelve() {
        assert_eq!(total_distance_sq(1), 12);
        assert_eq!(total_distance_sq(0), 0);
    }
}
