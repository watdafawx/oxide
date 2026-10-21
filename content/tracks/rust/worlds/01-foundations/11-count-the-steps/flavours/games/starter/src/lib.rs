/// Step 1 needs 1 block, step 2 needs 2, and so on. staircase_blocks(4) -> 1 + 2 + 3 + 4 = 10
pub fn staircase_blocks(steps: u32) -> u32 {
    // TODO: `let mut total = 0;`, a `for step in 1..=steps` loop that adds `step`, then `total`
    0
}

/// Layer 1 holds 1 block, layer 2 holds 4, layer 3 holds 9: each layer is `layer * layer`.
/// pyramid_blocks(3) -> 1 + 4 + 9 = 14
pub fn pyramid_blocks(layers: u32) -> u32 {
    // TODO: same shape as above, adding `layer * layer`
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_add_up_to_blocks() {
        assert_eq!(staircase_blocks(4), 10);
        assert_eq!(staircase_blocks(1), 1);
        assert_eq!(staircase_blocks(0), 0, "no steps, no blocks");
    }

    #[test]
    fn pyramid_layers_are_squares() {
        assert_eq!(pyramid_blocks(3), 14);
        assert_eq!(pyramid_blocks(1), 1);
        assert_eq!(pyramid_blocks(0), 0);
    }
}
