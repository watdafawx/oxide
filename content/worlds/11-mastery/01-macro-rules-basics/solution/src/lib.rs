/// `square!(x)` expands to `x * x`.
#[macro_export]
macro_rules! square {
    ($x:expr) => {{
        let v = $x;
        v * v
    }};
}

/// `hashmap!("a" => 1)` builds a HashMap with one pair.
#[macro_export]
macro_rules! hashmap {
    ($k:expr => $v:expr) => {{
        let mut m = ::std::collections::HashMap::new();
        m.insert($k, $v);
        m
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn square_multiplies() {
        assert_eq!(square!(4), 16);
        assert_eq!(square!(1 + 2), 9, "`1 + 2` is one expression: (1 + 2) * (1 + 2)");
    }

    #[test]
    fn square_evaluates_once() {
        let mut calls = 0;
        let mut next = || {
            calls += 1;
            2 + calls
        };
        let result = square!(next());
        assert_eq!(result, 9);
        assert_eq!(calls, 1, "the argument must be evaluated exactly once");
    }

    #[test]
    fn hashmap_builds_one_pair() {
        let m: HashMap<&str, i32> = hashmap!("a" => 1);
        assert_eq!(m.len(), 1);
        assert_eq!(m["a"], 1);
    }
}
