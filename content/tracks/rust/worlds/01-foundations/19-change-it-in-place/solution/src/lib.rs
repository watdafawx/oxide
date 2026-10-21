pub struct Pallet {
    pub label: String,
    pub boxes: u32,
}

impl Pallet {
    pub fn new(label: &str) -> Pallet {
        Pallet { label: label.to_string(), boxes: 64 }
    }

    /// Remove `amount` boxes. The count never goes below zero.
    pub fn take(&mut self, amount: u32) {
        self.boxes = self.boxes.saturating_sub(amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taking_removes_boxes() {
        let mut pallet = Pallet::new("paper");
        pallet.take(30);
        assert_eq!(pallet.boxes, 34);
    }

    #[test]
    fn taking_too_many_stops_at_zero() {
        let mut pallet = Pallet::new("paper");
        pallet.take(100);
        assert_eq!(pallet.boxes, 0, "boxes is a u32, so it must stop at 0");
    }
}
