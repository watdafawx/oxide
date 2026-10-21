pub struct Pallet {
    pub label: String,
    pub boxes: u32,
}

impl Pallet {
    /// A new pallet starts full: 64 boxes.
    pub fn new(label: &str) -> Pallet {
        Pallet { label: label.to_string(), boxes: 64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pallets_start_full() {
        let pallet = Pallet::new("paper");
        assert_eq!(pallet.boxes, 64);
    }

    #[test]
    fn new_keeps_the_label() {
        assert_eq!(Pallet::new("paper").label, "paper");
        assert_eq!(Pallet::new("glue").label, "glue");
    }
}
