/// How many items one craft produces. Unknown items have no recipe.
/// "planks" -> Some(4), "torch" -> Some(4), "diamond_pickaxe" -> Some(1), anything else -> None
pub fn recipe_output(item: &str) -> Option<u32> {
    match item {
        "planks" => Some(4),
        "torch" => Some(4),
        "diamond_pickaxe" => Some(1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_recipes_have_an_output() {
        assert_eq!(recipe_output("planks"), Some(4));
        assert_eq!(recipe_output("torch"), Some(4));
        assert_eq!(recipe_output("diamond_pickaxe"), Some(1));
    }

    #[test]
    fn unknown_items_have_no_recipe() {
        assert_eq!(recipe_output("bedrock"), None, "nobody can craft bedrock");
        assert_eq!(recipe_output(""), None);
    }
}
