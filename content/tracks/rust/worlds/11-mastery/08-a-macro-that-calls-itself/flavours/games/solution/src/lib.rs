/// `tallest!(64, 120, 90)` is 120: the biggest of one or more heights.
#[macro_export]
macro_rules! tallest {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {
        ::std::cmp::max($x, $crate::tallest!($($rest),+))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn tallest_of_one() {
        assert_eq!(tallest!(64), 64);
    }

    #[test]
    fn tallest_of_two() {
        assert_eq!(tallest!(64, 120), 120);
        assert_eq!(tallest!(120, 64), 120);
    }

    #[test]
    fn tallest_of_many() {
        assert_eq!(tallest!(12, 40, 63, 64, 200, 90, 70), 200);
    }
}
