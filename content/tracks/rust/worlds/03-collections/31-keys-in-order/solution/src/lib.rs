use std::collections::BTreeMap;

/// [pen, mug, pen] -> {mug: 1, pen: 2}, with the names sorted.
pub fn sales_report(sales: &[&str]) -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    for product in sales {
        *counts.entry(product.to_string()).or_insert(0) += 1;
    }
    counts
}

/// {mug: 1, pen: 2} -> ["mug: 1", "pen: 2"], a to z.
pub fn report_lines(counts: &BTreeMap<String, u32>) -> Vec<String> {
    counts.iter().map(|(name, n)| format!("{name}: {n}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sales_report_counts_in_a_sorted_map() {
        let report = sales_report(&["pen", "mug", "pen", "lamp"]);
        assert_eq!(report.get("pen"), Some(&2));
        let names: Vec<&String> = report.keys().collect();
        assert_eq!(names, vec!["lamp", "mug", "pen"], "a BTreeMap keeps the names sorted");
    }

    #[test]
    fn report_lines_walk_the_map_in_order() {
        let mut counts = BTreeMap::new();
        counts.insert("pen".to_string(), 2);
        counts.insert("mug".to_string(), 1);
        assert_eq!(report_lines(&counts), vec!["mug: 1", "pen: 2"]);
    }
}
