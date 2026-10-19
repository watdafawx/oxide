/// The n-th Fibonacci number: 0, 1, 1, 2, 3, 5, 8, ...
// TODO: above `pub fn fib`, add the attribute that exports it to JavaScript, but only when
// building for wasm32: `cfg_attr(<condition>, <attribute path>)`. See the hints.
pub fn fib(n: u32) -> u64 {
    // TODO: loop `n` times with two running numbers. Recursion would be far too slow for fib(90).
    todo!()
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
