/// square(5) -> 25
pub fn square(n: i32) -> i32 {
    // TODO: return n times n (no semicolon!)
    todo!()
}

/// area(3, 4) -> 12
pub fn area(w: u32, h: u32) -> u32 {
    // TODO: width times height
    todo!()
}

/// summary(3, 4) -> "3x4 = 12"
pub fn summary(w: u32, h: u32) -> String {
    // TODO: call area(w, h) and use format!
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_multiplies() {
        assert_eq!(square(5), 25);
        assert_eq!(square(-3), 9);
    }

    #[test]
    fn area_multiplies_sides() {
        assert_eq!(area(3, 4), 12);
        assert_eq!(area(0, 9), 0);
    }

    #[test]
    fn summary_reuses_area() {
        assert_eq!(summary(3, 4), "3x4 = 12");
        assert_eq!(summary(10, 2), "10x2 = 20");
    }
}
