/// Disk space one library takes, set once for the whole crate. Private.
const MB_PER_LIBRARY: u32 = 4;

pub mod resolver {
    /// How much disk `libraries` libraries need, in MB.
    pub fn library_mb(libraries: u32) -> u32 {
        // TODO: libraries times MB_PER_LIBRARY, which lives one room up
        todo!()
    }
}

// TODO: bring resolver::library_mb into this room with use

/// describe(5) -> "5 libraries need 20 MB"
pub fn describe(libraries: u32) -> String {
    // TODO: build the sentence, calling library_mb by its short name
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_mb_reads_the_constant_above() {
        assert_eq!(resolver::library_mb(5), 20);
        assert_eq!(resolver::library_mb(0), 0);
    }

    #[test]
    fn describe_calls_the_resolver() {
        assert_eq!(describe(5), "5 libraries need 20 MB");
    }
}
