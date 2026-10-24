pub struct Basket {
    pub counts: Vec<u32>,
    pub labels: Vec<String>,
}

/// Doubles every count in place.
pub fn double_all(basket: &mut Basket) {
    // TODO: lend each count mutably and double it
    todo!()
}

/// Adds " (new)" to the end of every label in place.
pub fn mark_all(basket: &mut Basket) {
    // TODO: lend each label mutably and add " (new)" to it
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basket() -> Basket {
        Basket {
            counts: vec![1, 4, 10],
            labels: vec!["jam".to_string(), "tea".to_string()],
        }
    }

    #[test]
    fn double_all_doubles_every_count() {
        let mut b = basket();
        double_all(&mut b);
        assert_eq!(b.counts, vec![2, 8, 20]);
    }

    #[test]
    fn mark_all_tags_every_label() {
        let mut b = basket();
        mark_all(&mut b);
        assert_eq!(b.labels, vec!["jam (new)", "tea (new)"]);
    }
}
