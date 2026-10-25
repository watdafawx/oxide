/// "Welcome, Ana!" on a first visit, "Welcome back, Ana!" after that.
pub fn welcome(name: &str, first_visit: bool) -> String {
    if first_visit {
        format!("Welcome, {name}!")
    } else {
        format!("Welcome back, {name}!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_visit_says_welcome() {
        assert_eq!(welcome("Ana", true), "Welcome, Ana!");
    }

    #[test]
    fn next_visit_says_welcome_back() {
        assert_eq!(welcome("Ana", false), "Welcome back, Ana!");
    }
}
