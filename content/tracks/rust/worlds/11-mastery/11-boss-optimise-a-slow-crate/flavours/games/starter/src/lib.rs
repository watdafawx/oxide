//! A small chunk index. It is correct and slow. Keep the public API, make it fast.
//!
//! - `ChunkIndex::build(log)`: index every block name of every chunk (one chunk per line, numbered from 1).
//! - `chunks_with(block)`: the chunks that contain `block`, ascending, each chunk once.
//! - `top_blocks(n)`: the `n` most frequent blocks as `(block, count)`, most frequent first,
//!   ties in alphabetical order.
//!
//! A block name is a run of letters and digits. Matching ignores case: "Stone" and "stone" are one block.

pub mod probe;

use probe::BlockName;

pub struct ChunkIndex {
    // One (block, chunks) pair per distinct block, in order of first appearance.
    entries: Vec<(BlockName, Vec<usize>)>,
}

impl ChunkIndex {
    pub fn build(log: &str) -> ChunkIndex {
        // TODO: for every token this allocates a lowercase String and scans every known block.
        // Look the block up in a `HashMap` instead, and reuse one buffer for lowercasing.
        let mut entries: Vec<(BlockName, Vec<usize>)> = Vec::new();
        for (i, line) in log.lines().enumerate() {
            let chunk_no = i + 1;
            for token in line.split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()) {
                let block = BlockName(token.to_lowercase());
                match entries.iter_mut().find(|(b, _)| *b == block) {
                    Some((_, chunks)) => chunks.push(chunk_no),
                    None => entries.push((block, vec![chunk_no])),
                }
            }
        }
        ChunkIndex { entries }
    }

    pub fn chunks_with(&self, block: &str) -> Vec<usize> {
        // TODO: this walks every known block. With a HashMap it is one lookup.
        let block = BlockName(block.to_lowercase());
        for (b, chunks) in &self.entries {
            if *b == block {
                let mut out = chunks.clone();
                out.dedup();
                return out;
            }
        }
        Vec::new()
    }

    pub fn top_blocks(&self, n: usize) -> Vec<(String, usize)> {
        // TODO: this clones every block name. Sort references, clone only the `n` you return.
        let mut all: Vec<(BlockName, usize)> =
            self.entries.iter().map(|(b, chunks)| (b.clone(), chunks.len())).collect();
        for i in 0..all.len().min(n) {
            let mut best = i;
            for j in i + 1..all.len() {
                let better = all[j].1 > all[best].1
                    || (all[j].1 == all[best].1 && all[j].0 < all[best].0);
                if better {
                    best = j;
                }
            }
            all.swap(i, best);
        }
        all.truncate(n);
        all.into_iter().map(|(b, count)| (b.0, count)).collect()
    }
}
