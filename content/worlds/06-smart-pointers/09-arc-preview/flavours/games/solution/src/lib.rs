use std::sync::Arc;
use std::thread;

/// A recipe with its raw material cost.
pub struct Recipe {
    pub name: String,
    pub cost: u32,
}

/// Adds up the cost of `recipes` using two threads that share the list through an Arc.
pub fn total_cost(recipes: Vec<Recipe>) -> u32 {
    let recipes = Arc::new(recipes);
    let mid = recipes.len() / 2;
    let first = Arc::clone(&recipes);
    let second = Arc::clone(&recipes);
    let a = thread::spawn(move || first[..mid].iter().map(|r| r.cost).sum::<u32>());
    let b = thread::spawn(move || second[mid..].iter().map(|r| r.cost).sum::<u32>());
    a.join().unwrap() + b.join().unwrap()
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
    fn parallel_sum_adds_everything() {
        let recipes: Vec<Recipe> = (1..=101).map(recipe).collect();
        assert_eq!(total_cost(recipes), 5151);
    }

    #[test]
    fn parallel_sum_handles_tiny_inputs() {
        assert_eq!(total_cost(Vec::new()), 0, "empty list");
        assert_eq!(total_cost(vec![recipe(7)]), 7, "one recipe");
    }
}
