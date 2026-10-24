/// The sum of every mod's version.
pub fn pack_fingerprint(lines: &[&str]) -> u32 {
    let mut total = 0;
    for line in lines {
        total += crate::manifest::version_of(line);
    }
    total
}
