pub struct Pallet {
    pub label: String,
    pub boxes: u32,
}

/// A pallet of paper with 64 boxes on it.
pub fn full_pallet() -> Pallet {
    Pallet { label: "paper".to_string(), boxes: 64 }
}

/// pallet_label(pallet) -> "paper x64"
pub fn pallet_label(pallet: Pallet) -> String {
    format!("{} x{}", pallet.label, pallet.boxes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_full_paper_pallet() {
        let pallet = full_pallet();
        assert_eq!(pallet.label, "paper");
        assert_eq!(pallet.boxes, 64);
    }

    #[test]
    fn label_shows_name_and_count() {
        assert_eq!(pallet_label(full_pallet()), "paper x64");
        let small = Pallet { label: "glue".to_string(), boxes: 3 };
        assert_eq!(pallet_label(small), "glue x3");
    }
}
