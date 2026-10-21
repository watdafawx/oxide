/// How many items one craft produces. Unknown items have no recipe.
pub fn recipe_output(item: &str) -> Option<u32> {
    match item {
        "planks" => Some(4),
        "torch" => Some(4),
        "diamond_pickaxe" => Some(1),
        _ => None,
    }
}

/// "planks makes 4", or "no recipe for bedrock" when there is none.
pub fn describe_recipe(item: &str) -> String {
    if let Some(n) = recipe_output(item) {
        format!("{item} makes {n}")
    } else {
        format!("no recipe for {item}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_recipes_are_described() {
        assert_eq!(describe_recipe("planks"), "planks makes 4");
        assert_eq!(describe_recipe("diamond_pickaxe"), "diamond_pickaxe makes 1");
    }

    #[test]
    fn unknown_recipes_are_reported() {
        assert_eq!(describe_recipe("bedrock"), "no recipe for bedrock");
    }
}
