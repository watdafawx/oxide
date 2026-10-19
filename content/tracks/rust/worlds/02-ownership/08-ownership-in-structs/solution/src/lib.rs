/// Owns its name.
#[derive(Debug, PartialEq)]
pub struct Owned {
    pub name: String,
    pub level: u32,
}

/// Borrows its name from somewhere else.
#[derive(Debug, PartialEq)]
pub struct Borrowed<'a> {
    pub name: &'a str,
    pub level: u32,
}

impl Owned {
    pub fn new(name: &str, level: u32) -> Owned {
        Owned {
            name: name.to_string(),
            level,
        }
    }

    /// A borrowed view of this Owned: same data, no copying.
    pub fn as_borrowed(&self) -> Borrowed<'_> {
        Borrowed {
            name: &self.name,
            level: self.level,
        }
    }
}

impl<'a> Borrowed<'a> {
    pub fn new(name: &'a str, level: u32) -> Borrowed<'a> {
        Borrowed { name, level }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_outlives_its_source() {
        let hero = {
            let temp = String::from("ferris");
            Owned::new(&temp, 3)
        };
        assert_eq!(hero.name, "ferris");
        assert_eq!(hero.level, 3);
    }

    #[test]
    fn borrowed_reads_from_source() {
        let text = String::from("ana");
        let view = Borrowed::new(&text, 7);
        assert_eq!(view.name, "ana");
        assert_eq!(view.level, 7);
        assert_eq!(view.name.as_ptr(), text.as_ptr(), "must point at the original text, not a copy");
    }

    #[test]
    fn as_borrowed_points_into_owned() {
        let hero = Owned::new("bo", 2);
        let view = hero.as_borrowed();
        assert_eq!(view, Borrowed { name: "bo", level: 2 });
        assert_eq!(view.name.as_ptr(), hero.name.as_ptr(), "must borrow, not copy");
    }
}
