use std::fmt::Display;

pub trait Convert {
    const NAME: &str;
    type Output;
    fn convert(&self) -> Self::Output;
}

pub struct Celsius(pub f64);

pub struct Words(pub String);

impl Convert for Celsius {
    const NAME: &str = "celsius->fahrenheit";
    type Output = f64;

    fn convert(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl Convert for Words {
    const NAME: &str = "word-count";
    type Output = usize;

    fn convert(&self) -> usize {
        self.0.split_whitespace().count()
    }
}

/// "<NAME>: <converted value>", e.g. "word-count: 3".
pub fn label<C>(item: &C) -> String
where
    C: Convert,
    C::Output: Display,
{
    format!("{}: {}", C::NAME, item.convert())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_converts_to_fahrenheit() {
        assert_eq!(Celsius(100.0).convert(), 212.0);
        assert_eq!(Celsius(0.0).convert(), 32.0);
    }

    #[test]
    fn words_counts_words() {
        assert_eq!(Words("one two  three".to_string()).convert(), 3);
        assert_eq!(Words(String::new()).convert(), 0);
    }

    #[test]
    fn names_are_set() {
        assert_eq!(Celsius::NAME, "celsius->fahrenheit");
        assert_eq!(Words::NAME, "word-count");
    }

    #[test]
    fn label_uses_name_and_output() {
        assert_eq!(label(&Words("a b c".to_string())), "word-count: 3");
        assert_eq!(label(&Celsius(100.0)), "celsius->fahrenheit: 212");
    }
}
