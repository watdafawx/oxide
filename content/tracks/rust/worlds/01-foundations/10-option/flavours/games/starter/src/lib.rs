/// How many items one craft produces. Unknown recipe -> None.
/// planks -> 4, stick -> 4, torch -> 4, diamond_pickaxe -> 1
pub fn recipe_output(item: &str) -> Option<u32> {
    // TODO: match on item; return Some(count) for known recipes and None for the rest
    todo!()
}

/// recipe_output, but 0 when the recipe is unknown.
pub fn output_or_zero(item: &str) -> u32 {
    // TODO: ask recipe_output, falling back to 0 when it returns None
    todo!()
}

/// A double batch: twice the output, or None when the recipe is unknown.
pub fn double_batch(item: &str) -> Option<u32> {
    // TODO: ask recipe_output, then double the count inside the Some (None stays None)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_output_is_none_for_unknown_items() {
        assert_eq!(recipe_output("planks"), Some(4));
        assert_eq!(recipe_output("diamond_pickaxe"), Some(1));
        assert_eq!(recipe_output("bedrock"), None, "nobody can craft bedrock");
    }

    #[test]
    fn output_or_zero_defaults_to_nothing() {
        assert_eq!(output_or_zero("torch"), 4);
        assert_eq!(output_or_zero("bedrock"), 0);
    }

    #[test]
    fn double_batch_maps_inside_some() {
        assert_eq!(double_batch("planks"), Some(8));
        assert_eq!(double_batch("bedrock"), None, "no recipe stays no recipe");
    }
}
