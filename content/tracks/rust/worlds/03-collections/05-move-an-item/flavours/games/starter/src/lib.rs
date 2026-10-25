/// Shift-click: take the item in `slot` and put it in slot 0. [sword, bow, pickaxe] with slot 2 -> [pickaxe, sword, bow].
pub fn move_to_hotbar(inventory: &mut Vec<String>, slot: usize) {
    // TODO: take the item out of `slot`, then put it in at position 0
}

/// Take the item in `slot` and put it last. [sword, bow, pickaxe] with slot 0 -> [bow, pickaxe, sword].
pub fn move_to_end(inventory: &mut Vec<String>, slot: usize) {
    // TODO: take the item out of `slot`, then add it at the end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inventory() -> Vec<String> {
        vec!["sword".to_string(), "bow".to_string(), "pickaxe".to_string()]
    }

    #[test]
    fn move_to_hotbar_puts_the_item_first() {
        let mut inv = inventory();
        move_to_hotbar(&mut inv, 2);
        assert_eq!(inv, vec!["pickaxe", "sword", "bow"]);
        move_to_hotbar(&mut inv, 0);
        assert_eq!(inv, vec!["pickaxe", "sword", "bow"], "slot 0 stays put");
    }

    #[test]
    fn move_to_end_puts_the_item_last() {
        let mut inv = inventory();
        move_to_end(&mut inv, 0);
        assert_eq!(inv, vec!["bow", "pickaxe", "sword"]);
    }
}
