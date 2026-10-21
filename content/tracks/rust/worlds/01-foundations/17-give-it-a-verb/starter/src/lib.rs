pub struct Pallet {
    pub label: String,
    pub boxes: u32,
}

impl Pallet {
    /// True when there are no boxes at all.
    pub fn is_empty(&self) -> bool {
        false // TODO: compare self.boxes with 0
    }

    /// True when the pallet holds exactly 64 boxes.
    pub fn is_full(&self) -> bool {
        false // TODO: compare self.boxes with 64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pallet(n: u32) -> Pallet {
        Pallet { label: "paper".to_string(), boxes: n }
    }

    #[test]
    fn an_empty_pallet_is_empty() {
        assert!(pallet(0).is_empty());
        assert!(!pallet(1).is_empty());
    }

    #[test]
    fn only_64_boxes_is_full() {
        assert!(pallet(64).is_full());
        assert!(!pallet(63).is_full());
        assert!(!pallet(0).is_full());
    }
}
