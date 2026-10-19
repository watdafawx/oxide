pub trait Greeter {
    fn name(&self) -> String;

    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }

    fn shout(&self) -> String {
        self.greet().to_uppercase()
    }
}

pub struct Cat;

impl Greeter for Cat {
    fn name(&self) -> String {
        "Cat".to_string()
    }
}

pub struct Parrot;

impl Greeter for Parrot {
    fn name(&self) -> String {
        "Parrot".to_string()
    }

    fn greet(&self) -> String {
        format!("Squawk! {} wants a cracker!", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_greet_uses_name() {
        assert_eq!(Cat.greet(), "Hello, Cat!");
    }

    #[test]
    fn shout_builds_on_greet() {
        assert_eq!(Cat.shout(), "HELLO, CAT!");
        assert_eq!(
            Parrot.shout(),
            "SQUAWK! PARROT WANTS A CRACKER!",
            "shout must call greet(), so overrides show through"
        );
    }

    #[test]
    fn parrot_overrides_greet() {
        assert_eq!(Parrot.greet(), "Squawk! Parrot wants a cracker!");
    }
}
