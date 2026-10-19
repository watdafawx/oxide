pub mod pack {
    /// Private: children can see it through `super::MB_PER_MOD`.
    const MB_PER_MOD: u32 = 3;

    pub mod manifest {
        /// Download size of `mods` mods: MB_PER_MOD each.
        pub fn size_mb(mods: u32) -> u32 {
            // TODO: use the parent's constant: super::MB_PER_MOD * mods
            todo!()
        }
    }

    pub mod resolver {
        /// Every library the mods pull in weighs 4 MB.
        pub fn library_mb(libs: u32) -> u32 {
            // TODO: libs * 4
            todo!()
        }
    }

    /// Size of `mods` mods plus `libs` libraries.
    pub fn total_mb(mods: u32, libs: u32) -> u32 {
        // TODO: self::manifest::size_mb(mods) + self::resolver::library_mb(libs)
        todo!()
    }
}

/// Text such as "download: 11 MB".
pub fn describe(mods: u32, libs: u32) -> String {
    // TODO: `use pack::total_mb;` (here or at the top of the file), then format!("download: {} MB", total_mb(mods, libs))
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_uses_parent_constant() {
        assert_eq!(pack::manifest::size_mb(2), 6, "MB_PER_MOD is 3 here, so 3 * 2");
    }

    #[test]
    fn square_area_multiplies_sides() {
        assert_eq!(pack::resolver::library_mb(4), 16);
    }

    #[test]
    fn total_area_adds_both_children() {
        assert_eq!(pack::total_mb(1, 2), 11, "3 for the mod + 8 for the libraries");
    }

    #[test]
    fn describe_uses_an_imported_function() {
        assert_eq!(describe(1, 2), "download: 11 MB");
    }
}
