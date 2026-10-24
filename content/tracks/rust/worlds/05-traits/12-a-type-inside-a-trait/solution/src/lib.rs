use std::fmt::Display;

pub trait Convert {
    type Output;
    fn convert(&self) -> Self::Output;
}

/// Degrees Celsius.
pub struct Celsius(pub f64);

/// Some text.
pub struct Words(pub String);

impl Convert for Celsius {
    type Output = f64;

    fn convert(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl Convert for Words {
    type Output = usize;

    fn convert(&self) -> usize {
        self.0.split_whitespace().count()
    }
}

/// "result: <converted value>", e.g. "result: 3".
pub fn report<C>(item: &C) -> String
where
    C: Convert,
    C::Output: Display,
{
    format!("result: {}", item.convert())
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
    fn report_prints_any_output() {
        assert_eq!(report(&Words("a b c".to_string())), "result: 3");
        assert_eq!(report(&Celsius(100.0)), "result: 212");
    }
}
