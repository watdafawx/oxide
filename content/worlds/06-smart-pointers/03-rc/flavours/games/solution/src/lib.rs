use std::rc::Rc;

/// What an assembler crafts. Many assemblers share one of these.
pub struct Recipe {
    pub name: String,
    pub craft_ticks: u32,
}

impl Recipe {
    pub fn new(name: &str, craft_ticks: u32) -> Recipe {
        Recipe {
            name: name.to_string(),
            craft_ticks,
        }
    }
}

/// Returns another owner of the same Recipe (no copy of it).
pub fn share_recipe(recipe: &Rc<Recipe>) -> Rc<Recipe> {
    Rc::clone(recipe)
}

/// How many assemblers use this recipe right now?
pub fn assembler_count(recipe: &Rc<Recipe>) -> usize {
    Rc::strong_count(recipe)
}

/// Returns (count while a temporary extra assembler exists, count after it is gone).
pub fn count_with_extra_assembler(recipe: &Rc<Recipe>) -> (usize, usize) {
    let inside = {
        let extra = Rc::clone(recipe);
        Rc::strong_count(&extra)
    };
    (inside, Rc::strong_count(recipe))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_returns_another_handle() {
        let a = Rc::new(Recipe::new("iron-gear-wheel", 30));
        let b = share_recipe(&a);
        assert!(Rc::ptr_eq(&a, &b), "both assemblers must use the same Recipe");
        assert_eq!(Rc::strong_count(&a), 2);
    }

    #[test]
    fn owner_count_reports_strong_count() {
        let a = Rc::new(Recipe::new("iron-gear-wheel", 30));
        let _b = Rc::clone(&a);
        let _c = Rc::clone(&a);
        assert_eq!(assembler_count(&a), 3);
    }

    #[test]
    fn count_inside_and_after_tracks_drops() {
        let a = Rc::new(Recipe::new("iron-gear-wheel", 30));
        assert_eq!(count_with_extra_assembler(&a), (2, 1));
    }
}
