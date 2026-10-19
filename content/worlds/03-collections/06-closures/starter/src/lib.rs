/// make_adder(5) returns a closure; calling it with 3 gives 8.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // TODO: return a closure: `move |x| ...` that adds n to x
       move |x| x
}

/// Call `f` on `x`, then call `f` on that result. apply_twice(|n| n * 3, 2) = 18.
pub fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    // TODO: f(f(x))
    todo!()
}

/// How many items make `is_match` return true?
pub fn count_matching(items: &[i32], is_match: impl Fn(i32) -> bool) -> usize {
    // TODO: items.iter().filter(|n| ...).count(); call is_match with the number
    todo!()
}

/// Keep numbers strictly greater than `limit`: ([1, 5, 9], 4) -> [5, 9]
pub fn above_limit(numbers: &[i32], limit: i32) -> Vec<i32> {
    // TODO: a filter closure that uses `limit` straight from this function
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_adder_captures_n() {
        let add5 = make_adder(5);
        assert_eq!(add5(3), 8);
        assert_eq!(make_adder(-1)(0), -1);
    }

    #[test]
    fn apply_twice_calls_f_twice() {
        assert_eq!(apply_twice(|n| n * 3, 2), 18);
        assert_eq!(apply_twice(|n| n + 1, 0), 2);
    }

    #[test]
    fn count_matching_uses_predicate() {
        assert_eq!(count_matching(&[1, 2, 3, 4, 5], |n| n > 2), 3);
        assert_eq!(count_matching(&[1, 2, 3, 4, 5], |n| n % 2 == 0), 2);
        assert_eq!(count_matching(&[], |_| true), 0);
    }

    #[test]
    fn above_limit_captures_limit() {
        assert_eq!(above_limit(&[1, 5, 9], 4), vec![5, 9]);
        assert_eq!(above_limit(&[1, 5, 9], 9), Vec::<i32>::new());
        assert_eq!(above_limit(&[1, 5, 9], 0), vec![1, 5, 9]);
    }
}
