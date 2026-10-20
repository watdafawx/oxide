pub mod pack {
    /// Private: children can see it through `super::MB_PER_MOD`.
    const MB_PER_MOD: u32 = 3;

    pub mod manifest {
        /// Download size of `mods` mods: MB_PER_MOD each.
        pub fn size_mb(mods: u32) -> u32 {
            // TODO: size of the mods, using the parent module's private constant
            todo!()
        }
    }

    pub mod resolver {
        /// Every library the mods pull in weighs 4 MB.
        pub fn library_mb(libs: u32) -> u32 {
            // TODO: each library is 4 MB
            todo!()
        }
    }

    /// Size of `mods` mods plus `libs` libraries.
    pub fn total_mb(mods: u32, libs: u32) -> u32 {
        // TODO: add the manifest's size to the resolver's library size, calling into the child modules
        todo!()
    }
}

/// Text such as "download: 11 MB".
pub fn describe(mods: u32, libs: u32) -> String {
    // TODO: bring `total_mb` into scope with `use`, then format "download: <total> MB"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_mb_uses_parent_constant() {
        assert_eq!(pack::manifest::size_mb(2), 6, "MB_PER_MOD is 3 here, so 3 * 2");
    }

    #[test]
    fn library_mb_adds_up_mods() {
        assert_eq!(pack::resolver::library_mb(4), 16);
    }

    #[test]
    fn total_mb_adds_both_children() {
        assert_eq!(pack::total_mb(1, 2), 11, "3 for the mod + 8 for the libraries");
    }

    #[test]
    fn describe_uses_imported_total_mb() {
        assert_eq!(describe(1, 2), "download: 11 MB");
    }
}
