/// How many items one craft produces. Unknown recipe -> None.
/// planks -> 4, stick -> 4, torch -> 4, diamond_pickaxe -> 1
pub fn recipe_output(item: &str) -> Option<u32> {
    // TODO: match on item; return Some(count) for known recipes and None for the rest
    todo!()
}

/// recipe_output, but 0 when the recipe is unknown.
pub fn output_or_zero(item: &str) -> u32 {
    // TODO: build on recipe_output with `unwrap_or(0)`
    todo!()
}

/// A double batch: twice the output, or None when the recipe is unknown.
pub fn double_batch(item: &str) -> Option<u32> {
    // TODO: build on recipe_output with `map(|n| n * 2)`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_is_none_for_odd() {
        assert_eq!(recipe_output("planks"), Some(4));
        assert_eq!(recipe_output("diamond_pickaxe"), Some(1));
        assert_eq!(recipe_output("bedrock"), None, "nobody can craft bedrock");
    }

    #[test]
    fn half_or_zero_falls_back() {
        assert_eq!(output_or_zero("torch"), 4);
        assert_eq!(output_or_zero("bedrock"), 0);
    }

    #[test]
    fn half_plus_one_maps_inside() {
        assert_eq!(double_batch("planks"), Some(8));
        assert_eq!(double_batch("bedrock"), None, "no recipe stays no recipe");
    }
}
