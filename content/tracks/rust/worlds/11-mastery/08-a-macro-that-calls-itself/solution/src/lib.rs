/// `max!(3, 9, 2)` is 9: the biggest of one or more arguments.
#[macro_export]
macro_rules! max {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {
        ::std::cmp::max($x, $crate::max!($($rest),+))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn max_of_one() {
        assert_eq!(max!(5), 5);
    }

    #[test]
    fn max_of_two() {
        assert_eq!(max!(3, 9), 9);
        assert_eq!(max!(9, 3), 9);
    }

    #[test]
    fn max_of_many() {
        assert_eq!(max!(1, 2, 3, 4, 8, 7, 6), 8);
    }
}
