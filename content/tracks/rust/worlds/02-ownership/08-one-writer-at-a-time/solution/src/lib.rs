/// Adds `item` to the end of the list and reports what is first.
/// ["rope"] + "lamp" -> "first: rope, now 2 items"
pub fn add_and_report(items: &mut Vec<String>, item: &str) -> String {
    items.push(item.to_string());
    let first = &items[0];
    format!("first: {first}, now {} items", items.len())
}

/// Moves the last item of `from` to the end of `to`. Does nothing if `from` is empty.
pub fn move_last(from: &mut Vec<String>, to: &mut Vec<String>) {
    if let Some(x) = from.pop() {
        to.push(x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_report_names_the_first_item() {
        let mut items = vec!["rope".to_string()];
        assert_eq!(add_and_report(&mut items, "lamp"), "first: rope, now 2 items");
        assert_eq!(items, vec!["rope", "lamp"]);
    }

    #[test]
    fn move_last_between_vecs() {
        let mut a = vec!["rope".to_string(), "lamp".to_string()];
        let mut b = vec!["map".to_string()];
        move_last(&mut a, &mut b);
        assert_eq!(a, vec!["rope"]);
        assert_eq!(b, vec!["map", "lamp"]);
        let mut empty: Vec<String> = Vec::new();
        move_last(&mut empty, &mut b);
        assert_eq!(b, vec!["map", "lamp"], "nothing to move from an empty list");
    }
}
