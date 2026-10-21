/// Row 1 has 1 seat, row 2 has 2, and so on. seats_in_rows(4) -> 1 + 2 + 3 + 4 = 10
pub fn seats_in_rows(rows: u32) -> u32 {
    let mut total = 0;
    for row in 1..=rows {
        total += row;
    }
    total
}

/// Layer 1 holds 1 box, layer 2 holds 4, layer 3 holds 9: each layer is `layer * layer`.
/// boxes_in_pyramid(3) -> 1 + 4 + 9 = 14
pub fn boxes_in_pyramid(layers: u32) -> u32 {
    let mut total = 0;
    for layer in 1..=layers {
        total += layer * layer;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_add_up_to_seats() {
        assert_eq!(seats_in_rows(4), 10);
        assert_eq!(seats_in_rows(1), 1);
        assert_eq!(seats_in_rows(0), 0, "no rows, no seats");
    }

    #[test]
    fn pyramid_layers_are_squares() {
        assert_eq!(boxes_in_pyramid(3), 14);
        assert_eq!(boxes_in_pyramid(1), 1);
        assert_eq!(boxes_in_pyramid(0), 0);
    }
}
