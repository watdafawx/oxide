/// A fragile pallet holds 10 boxes, any other pallet 64.
pub fn pallet_limit(is_fragile: bool) -> u32 {
    if is_fragile { 10 } else { 64 }
}

/// Up to 1 kg costs 5, up to 5 kg costs 8, anything heavier costs 12.
pub fn shipping_fee(weight: u32) -> u32 {
    if weight <= 1 {
        5
    } else if weight <= 5 {
        8
    } else {
        12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragile_pallets_hold_less() {
        assert_eq!(pallet_limit(true), 10);
        assert_eq!(pallet_limit(false), 64);
    }

    #[test]
    fn fee_grows_with_weight() {
        assert_eq!(shipping_fee(1), 5);
        assert_eq!(shipping_fee(3), 8);
        assert_eq!(shipping_fee(5), 8);
        assert_eq!(shipping_fee(6), 12);
    }
}
