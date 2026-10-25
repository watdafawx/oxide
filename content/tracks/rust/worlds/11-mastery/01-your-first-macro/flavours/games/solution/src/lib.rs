/// `chunk_area!(side)` is `side * side`, for any number type.
#[macro_export]
macro_rules! chunk_area {
    ($x:expr) => {
        $x * $x
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn area_of_whole_blocks() {
        assert_eq!(chunk_area!(16), 256);
        assert_eq!(chunk_area!(8 + 8), 256, "`8 + 8` is one expression: (8 + 8) * (8 + 8)");
    }

    #[test]
    fn area_of_half_slabs() {
        assert_eq!(chunk_area!(0.5), 0.25);
    }
}
