use std::fmt::Display;

/// The value as uppercase text with a "!" at the end: shout("hi") is "HI!".
pub fn shout(x: impl Display) -> String {
    format!("{}!", x.to_string().to_uppercase())
}

/// Even numbers below `limit`: 0, 2, 4, ...
pub fn evens(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|n| n % 2 == 0)
}

/// A closure that adds `n` to its input.
pub fn adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// A closure that runs `f` first, then `g` on the result.
pub fn compose(f: impl Fn(i32) -> i32, g: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shout_accepts_anything_displayable() {
        assert_eq!(shout("hi"), "HI!");
        assert_eq!(shout(42), "42!");
        assert_eq!(shout(String::from("ok")), "OK!");
    }

    #[test]
    fn evens_returns_an_iterator() {
        let v: Vec<u32> = evens(7).collect();
        assert_eq!(v, vec![0, 2, 4, 6]);
        assert_eq!(evens(7).count(), 4);
    }

    #[test]
    fn adder_returns_closure() {
        let add5 = adder(5);
        assert_eq!(add5(1), 6);
        assert_eq!(adder(-2)(10), 8);
    }

    #[test]
    fn compose_chains_closures() {
        let f = compose(adder(1), |x| x * 10);
        assert_eq!(f(2), 30, "(2 + 1) * 10");
        let g = compose(|x| x * 10, adder(1));
        assert_eq!(g(2), 21, "2 * 10 + 1");
    }
}
