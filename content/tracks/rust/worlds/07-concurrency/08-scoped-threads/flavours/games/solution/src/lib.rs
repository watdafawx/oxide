use std::thread;

/// One jar from the mods folder.
#[derive(Debug, Clone)]
pub struct ModJar {
    pub name: String,
    pub bytes: u64,
}

impl ModJar {
    pub fn new(name: &str, bytes: u64) -> ModJar {
        ModJar {
            name: name.to_string(),
            bytes,
        }
    }
}

/// Stand-in for a real hash: given, do not change.
pub fn quick_hash(jar: &ModJar) -> u64 {
    jar.bytes * 31 + jar.name.len() as u64
}

/// Adds up the sizes using two scoped threads (first half, second half).
pub fn total_size(sizes: &[u64]) -> u64 {
    let (left, right) = sizes.split_at(sizes.len() / 2);
    thread::scope(|s| {
        let a = s.spawn(|| left.iter().sum::<u64>());
        let b = s.spawn(|| right.iter().sum::<u64>());
        a.join().unwrap() + b.join().unwrap()
    })
}

/// Size of the biggest jar, found by one scoped thread per chunk of `chunk_size`
/// jars. Returns None for an empty slice. `chunk_size` is at least 1.
pub fn biggest_jar(jars: &[ModJar], chunk_size: usize) -> Option<u64> {
    thread::scope(|s| {
        let handles: Vec<_> = jars
            .chunks(chunk_size)
            .map(|chunk| s.spawn(move || chunk.iter().map(|j| j.bytes).max()))
            .collect();
        handles.into_iter().filter_map(|h| h.join().unwrap()).max()
    })
}

/// Writes `quick_hash(&jars[i])` into `out[i]`. Two threads: one for each half.
/// `jars` and `out` have the same length.
pub fn hash_into(jars: &[ModJar], out: &mut [u64]) {
    let mid = jars.len() / 2;
    let (jars_left, jars_right) = jars.split_at(mid);
    let (out_left, out_right) = out.split_at_mut(mid);
    thread::scope(|s| {
        s.spawn(|| {
            for (jar, slot) in jars_left.iter().zip(out_left.iter_mut()) {
                *slot = quick_hash(jar);
            }
        });
        s.spawn(|| {
            for (jar, slot) in jars_right.iter().zip(out_right.iter_mut()) {
                *slot = quick_hash(jar);
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_size_borrows_both_halves() {
        let sizes: Vec<u64> = (1..=1000).collect();
        assert_eq!(total_size(&sizes), 500_500);
        assert_eq!(total_size(&[]), 0);
        assert_eq!(total_size(&[5]), 5);
        assert_eq!(total_size(&sizes[..7]), 28);
    }

    #[test]
    fn biggest_jar_joins_inside_the_scope() {
        let jars: Vec<ModJar> = [30, 90, 20, 70, 10, 80]
            .iter()
            .map(|&b| ModJar::new("x.jar", b))
            .collect();
        assert_eq!(biggest_jar(&jars, 2), Some(90));
        assert_eq!(biggest_jar(&jars, 1), Some(90));
        assert_eq!(biggest_jar(&jars[2..3], 10), Some(20));
        assert_eq!(biggest_jar(&[], 3), None);
    }

    #[test]
    fn hash_into_writes_disjoint_halves() {
        let jars: Vec<ModJar> = (0..7).map(|i| ModJar::new("m.jar", i * 100)).collect();
        let mut out = vec![0u64; 7];
        hash_into(&jars, &mut out);
        let expected: Vec<u64> = jars.iter().map(quick_hash).collect();
        assert_eq!(out, expected);

        let mut empty: Vec<u64> = Vec::new();
        hash_into(&[], &mut empty);
        assert!(empty.is_empty());
    }
}
