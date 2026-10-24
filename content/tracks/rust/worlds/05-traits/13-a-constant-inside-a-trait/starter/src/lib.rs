use std::fmt::Display;

pub trait Convert {
    const NAME: &str;
    type Output;
    fn convert(&self) -> Self::Output;
}

pub struct Celsius(pub f64);

pub struct Words(pub String);

impl Convert for Celsius {
    // TODO: set NAME to "celsius->fahrenheit"
    const NAME: &str = "";
    type Output = f64;

    fn convert(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl Convert for Words {
    // TODO: set NAME to "word-count"
    const NAME: &str = "";
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
    // TODO: return "<NAME>: <output>", reading NAME from the type C
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

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
