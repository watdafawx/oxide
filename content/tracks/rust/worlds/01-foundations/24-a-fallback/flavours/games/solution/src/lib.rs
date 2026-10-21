/// How many items one craft produces. Unknown items have no recipe.
pub fn recipe_output(item: &str) -> Option<u32> {
    match item {
        "planks" => Some(4),
        "torch" => Some(4),
        "diamond_pickaxe" => Some(1),
        _ => None,
    }
}

/// recipe_output, but 0 when the recipe is unknown.
pub fn output_or_zero(item: &str) -> u32 {
    recipe_output(item).unwrap_or(0)
}

/// How many items `crafts` crafts of `item` make. Unknown items make 0.
/// total_output("planks", 3) -> 12
pub fn total_output(item: &str, crafts: u32) -> u32 {
    recipe_output(item).unwrap_or(0) * crafts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_recipes_give_their_count() {
        assert_eq!(output_or_zero("planks"), 4);
        assert_eq!(output_or_zero("diamond_pickaxe"), 1);
        assert_eq!(output_or_zero("bedrock"), 0);
    }

    #[test]
    fn crafts_multiply_the_count() {
        assert_eq!(total_output("planks", 3), 12);
        assert_eq!(total_output("torch", 2), 8);
        assert_eq!(total_output("bedrock", 5), 0);
    }
}
