use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

/// `{:?}` text, e.g. `ChunkPos { x: 3, z: -2 }`.
pub fn debug_text(p: &ChunkPos) -> String {
    format!("{p:?}")
}

/// Are the two chunks the same?
pub fn is_same(a: &ChunkPos, b: &ChunkPos) -> bool {
    a == b
}

/// A copy of `p` with its x replaced.
pub fn shifted_to_x(p: &ChunkPos, x: i32) -> ChunkPos {
    let mut q = p.clone();
    q.x = x;
    q
}

/// The chunk at the world origin: x = 0, z = 0.
pub fn origin_chunk() -> ChunkPos {
    ChunkPos::default()
}

impl fmt::Display for ChunkPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.z)
    }
}

impl From<(i32, i32)> for ChunkPos {
    fn from(xz: (i32, i32)) -> Self {
        ChunkPos { x: xz.0, z: xz.1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spawn() -> ChunkPos {
        ChunkPos { x: 3, z: -2 }
    }

    #[test]
    fn chunk_pos_debug_and_equality() {
        assert_eq!(debug_text(&spawn()), "ChunkPos { x: 3, z: -2 }");
        assert!(is_same(&spawn(), &spawn()), "same x and z are equal");
        let mut other = spawn();
        other.z = -1;
        assert!(!is_same(&spawn(), &other), "different z is not equal");
    }

    #[test]
    fn chunk_pos_clone_and_default() {
        let moved = shifted_to_x(&spawn(), 10);
        assert_eq!(moved.x, 10);
        assert_eq!(moved.z, -2);
        let origin = origin_chunk();
        assert_eq!(origin.x, 0);
        assert_eq!(origin.z, 0);
    }

    #[test]
    fn chunk_pos_displays_as_pair() {
        assert_eq!(spawn().to_string(), "[3, -2]");
        assert_eq!(format!("{}", ChunkPos { x: 0, z: 7 }), "[0, 7]");
    }

    #[test]
    fn chunk_pos_from_tuple() {
        let p = ChunkPos::from((3, -2));
        assert_eq!(p.x, 3);
        assert_eq!(p.z, -2);
        let q: ChunkPos = (5, 6).into();
        assert_eq!(q.z, 6, ".into() comes free with From");
    }
}
