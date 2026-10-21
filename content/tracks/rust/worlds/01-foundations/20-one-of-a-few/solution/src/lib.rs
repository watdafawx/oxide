pub enum Mail {
    Letter,
    Parcel,
    Freight,
}

/// How many fit on one pallet: letters 64, parcels 16, freight 1.
pub fn max_per_pallet(kind: Mail) -> u32 {
    match kind {
        Mail::Letter => 64,
        Mail::Parcel => 16,
        Mail::Freight => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_and_parcels_stack() {
        assert_eq!(max_per_pallet(Mail::Letter), 64);
        assert_eq!(max_per_pallet(Mail::Parcel), 16);
    }

    #[test]
    fn freight_travels_alone() {
        assert_eq!(max_per_pallet(Mail::Freight), 1);
    }
}
