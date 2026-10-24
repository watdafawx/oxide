/// Every price with `percent_off` percent taken off: ([100, 500], 20) -> [80, 400]
pub fn with_discount(prices: &[u32], percent_off: u32) -> Vec<u32> {
    // TODO: one chain: walk the prices, take the percent off each one, collect into a Vec
    Vec::new()
}

/// Every product name in capitals, for the shelf signs: [pen, mug] -> [PEN, MUG]
pub fn sign_labels(names: &[String]) -> Vec<String> {
    // TODO: one chain: walk the names, turn each into capitals with to_uppercase, collect
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_discount_lowers_every_price() {
        assert_eq!(with_discount(&[100, 500], 20), vec![80, 400]);
        assert_eq!(with_discount(&[250], 0), vec![250], "0% off changes nothing");
    }

    #[test]
    fn sign_labels_are_in_capitals() {
        let names = vec!["pen".to_string(), "mug".to_string()];
        assert_eq!(sign_labels(&names), vec!["PEN", "MUG"]);
        assert_eq!(names, vec!["pen", "mug"], "the original list is untouched");
    }
}
