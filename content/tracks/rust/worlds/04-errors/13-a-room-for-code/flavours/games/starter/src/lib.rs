mod pack {
    /// A pack holds at most this many mods. Private: only code in `pack` sees it.
    const MAX_MODS: u32 = 200;

    // TODO: let code outside this module call slots_left
    fn slots_left(installed: u32) -> u32 {
        MAX_MODS.saturating_sub(installed)
    }
}

/// Can one more mod go in, when the pack already has `installed`?
pub fn can_install(installed: u32) -> bool {
    // TODO: ask the pack how many slots are left; true when it is above 0
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_left_is_open_to_callers() {
        assert_eq!(pack::slots_left(198), 2);
        assert_eq!(pack::slots_left(250), 0);
    }

    #[test]
    fn can_install_asks_the_pack() {
        assert!(can_install(10));
        assert!(can_install(199), "one slot left");
        assert!(!can_install(200), "the pack is full");
    }
}
