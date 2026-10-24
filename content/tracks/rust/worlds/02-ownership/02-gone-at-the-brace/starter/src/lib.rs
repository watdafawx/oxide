/// "Welcome, Ana!" on a first visit, "Welcome back, Ana!" after that.
pub fn welcome(name: &str, first_visit: bool) -> String {
    if first_visit {
        let greeting = format!("Welcome, {name}!");
    } else {
        let greeting = format!("Welcome back, {name}!");
    }
    // TODO: this does not build: each `greeting` is gone at the `}` of its branch.
    // Make the `if` itself give back the text.
    greeting
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
