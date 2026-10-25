/// Remove every sold-out count: [12, 0, 6, 0] -> [12, 6]
pub fn drop_sold_out(counts: &mut Vec<u32>) {
    // TODO: keep only the counts above 0
}

/// Remove every entry named `gone`: ([pen, fax, mug], "fax") -> [pen, mug]
pub fn drop_discontinued(products: &mut Vec<String>, gone: &str) {
    // TODO: keep every product whose name is not `gone`
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_sold_out_removes_the_zeros() {
        let mut counts = vec![12, 0, 6, 0];
        drop_sold_out(&mut counts);
        assert_eq!(counts, vec![12, 6]);
    }

    #[test]
    fn drop_discontinued_removes_one_product() {
        let mut products = vec!["pen".to_string(), "fax".to_string(), "mug".to_string(), "fax".to_string()];
        drop_discontinued(&mut products, "fax");
        assert_eq!(products, vec!["pen", "mug"], "both fax entries go");
    }
}
