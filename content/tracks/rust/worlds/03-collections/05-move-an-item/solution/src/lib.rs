/// Take the product at `index` and put it first. [pen, mug, lamp] with index 2 -> [lamp, pen, mug].
pub fn move_to_front(shelf: &mut Vec<String>, index: usize) {
    let product = shelf.remove(index);
    shelf.insert(0, product);
}

/// Take the product at `index` and put it last. [pen, mug, lamp] with index 0 -> [mug, lamp, pen].
pub fn move_to_back(shelf: &mut Vec<String>, index: usize) {
    let product = shelf.remove(index);
    shelf.push(product);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shelf() -> Vec<String> {
        vec!["pen".to_string(), "mug".to_string(), "lamp".to_string()]
    }

    #[test]
    fn move_to_front_puts_the_product_first() {
        let mut s = shelf();
        move_to_front(&mut s, 2);
        assert_eq!(s, vec!["lamp", "pen", "mug"]);
        move_to_front(&mut s, 0);
        assert_eq!(s, vec!["lamp", "pen", "mug"], "the first one stays first");
    }

    #[test]
    fn move_to_back_puts_the_product_last() {
        let mut s = shelf();
        move_to_back(&mut s, 0);
        assert_eq!(s, vec!["mug", "lamp", "pen"]);
    }
}
