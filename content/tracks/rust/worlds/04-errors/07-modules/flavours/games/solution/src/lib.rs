pub mod pack {
    /// Private: children can see it through `super::MB_PER_MOD`.
    const MB_PER_MOD: u32 = 3;

    pub mod manifest {
        /// Download size of `mods` mods: MB_PER_MOD each.
        pub fn size_mb(mods: u32) -> u32 {
            super::MB_PER_MOD * mods
        }
    }

    pub mod resolver {
        /// Every library the mods pull in weighs 4 MB.
        pub fn library_mb(libs: u32) -> u32 {
            libs * 4
        }
    }

    /// Size of `mods` mods plus `libs` libraries.
    pub fn total_mb(mods: u32, libs: u32) -> u32 {
        self::manifest::size_mb(mods) + self::resolver::library_mb(libs)
    }
}

/// Text such as "download: 11 MB".
pub fn describe(mods: u32, libs: u32) -> String {
    use pack::total_mb;
    format!("download: {} MB", total_mb(mods, libs))
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
