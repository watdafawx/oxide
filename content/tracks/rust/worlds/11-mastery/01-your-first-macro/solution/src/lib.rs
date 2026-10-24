/// `square!(x)` is `x * x`, for any number type.
#[macro_export]
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn square_of_whole_numbers() {
        assert_eq!(square!(4), 16);
        assert_eq!(square!(1 + 2), 9, "`1 + 2` is one expression: (1 + 2) * (1 + 2)");
    }

    #[test]
    fn square_of_decimals() {
        assert_eq!(square!(1.5), 2.25);
    }
}
