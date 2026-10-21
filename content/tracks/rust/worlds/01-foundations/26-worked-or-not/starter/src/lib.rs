/// Accept a pallet of up to 64 boxes.
/// check_pallet(32) -> Ok(32), check_pallet(100) -> Err("100 is over the pallet limit of 64")
pub fn check_pallet(boxes: u32) -> Result<u32, String> {
    // TODO: if boxes > 64 { Err(format!(..)) } else { Ok(boxes) }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_valid_pallet_is_ok() {
        assert_eq!(check_pallet(32), Ok(32));
        assert_eq!(check_pallet(64), Ok(64), "exactly 64 is a full pallet, not an error");
    }

    #[test]
    fn an_oversized_pallet_is_err() {
        assert_eq!(
            check_pallet(100),
            Err("100 is over the pallet limit of 64".to_string())
        );
    }
}
