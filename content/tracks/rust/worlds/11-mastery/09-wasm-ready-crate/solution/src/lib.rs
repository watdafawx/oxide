/// The n-th Fibonacci number: 0, 1, 1, 2, 3, 5, 8, ...
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_small_values() {
        let got: Vec<u64> = (0..=10).map(fib).collect();
        assert_eq!(got, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }

    #[test]
    fn fib_large_value_does_not_overflow() {
        assert_eq!(fib(90), 2_880_067_194_370_816_120);
    }

    #[test]
    fn fib_is_exported_to_wasm() {
        // Look at the source above the tests, ignoring comments and spaces.
        let source = include_str!("lib.rs");
        let code = source.split("#[cfg(test)]").next().unwrap();
        let compact: String = code
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .flat_map(|l| l.chars().filter(|c| !c.is_whitespace()))
            .collect();
        assert!(
            compact.contains("cfg_attr(target_arch=\"wasm32\",") && compact.contains("wasm_bindgen"),
            "put a `cfg_attr(target_arch = \"wasm32\", ...)` attribute with wasm_bindgen on `fib`"
        );
    }
}
