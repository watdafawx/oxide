#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

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
        if self.x > self.last_x {
            None
        } else {
            let chunk = ChunkPos { x: self.x, z: self.z };
            self.x += 1;
            Some(chunk)
        }
    }
}

/// The chunks of the row whose x is even (a toy slime-chunk rule).
pub fn slime_chunks(first_x: i32, last_x: i32, z: i32) -> Vec<ChunkPos> {
    // TODO: filter the row to chunks with an even x and collect them
    // (x can be negative: test evenness with c.x.rem_euclid(2) == 0, since -3 % 2 is -1)
    Vec::new()
}

/// x * x + z * z of every chunk in the row, added up.
pub fn total_distance_sq(first_x: i32, last_x: i32, z: i32) -> i32 {
    // TODO: map each chunk of the row to x * x + z * z and add them up
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slime_chunks_keep_even_x() {
        let v = slime_chunks(-3, 2, 7);
        let xs: Vec<i32> = v.iter().map(|c| c.x).collect();
        assert_eq!(xs, vec![-2, 0, 2]);
        assert!(v.iter().all(|c| c.z == 7));
    }

    #[test]
    fn total_distance_sq_adds_up_the_row() {
        assert_eq!(total_distance_sq(-2, 2, 1), 15, "(4+1) + (1+1) + (0+1) + (1+1) + (4+1)");
        assert_eq!(total_distance_sq(3, 2, 0), 0, "an empty row");
    }
}
