use std::sync::Arc;
use std::thread;

/// A recipe with its raw material cost.
pub struct Recipe {
    pub name: String,
    pub cost: u32,
}

/// Adds up the cost of `recipes` using two threads that share the list through an Arc.
pub fn total_cost(recipes: Vec<Recipe>) -> u32 {
    // TODO: let recipes = Arc::new(recipes); let mid = recipes.len() / 2;
    // clone the Arc once per thread, thread::spawn(move || ...) for each half,
    // then join() both and add them
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recipe(cost: u32) -> Recipe {
        Recipe {
            name: format!("recipe-{cost}"),
            cost,
        }
    }

    #[test]
    fn total_cost_adds_both_halves() {
        let recipes: Vec<Recipe> = (1..=101).map(recipe).collect();
        assert_eq!(total_cost(recipes), 5151);
    }

    #[test]
    fn total_cost_handles_tiny_lists() {
        assert_eq!(total_cost(Vec::new()), 0, "empty list");
        assert_eq!(total_cost(vec![recipe(7)]), 7, "one recipe");
    }
}
