//! A small chunk index. It is correct and slow. Keep the public API, make it fast.
//!
//! - `ChunkIndex::build(log)`: index every block name of every chunk (one chunk per line, numbered from 1).
//! - `chunks_with(block)`: the chunks that contain `block`, ascending, each chunk once.
//! - `top_blocks(n)`: the `n` most frequent blocks as `(block, count)`, most frequent first,
//!   ties in alphabetical order.
//!
//! A block name is a run of letters and digits. Matching ignores case: "Stone" and "stone" are one block.

pub mod probe;

use std::collections::HashMap;

struct Entry {
    chunks: Vec<usize>,
    count: usize,
}

pub struct ChunkIndex {
    blocks: HashMap<String, Entry>,
}

impl ChunkIndex {
    pub fn build(log: &str) -> ChunkIndex {
        let mut blocks: HashMap<String, Entry> = HashMap::new();
        let mut buf = String::new();
        for (i, line) in log.lines().enumerate() {
            let chunk_no = i + 1;
            for token in line.split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()) {
                buf.clear();
                buf.extend(token.chars().flat_map(char::to_lowercase));
                match blocks.get_mut(buf.as_str()) {
                    Some(entry) => {
                        entry.count += 1;
                        if entry.chunks.last() != Some(&chunk_no) {
                            entry.chunks.push(chunk_no);
                        }
                    }
                    None => {
                        blocks.insert(buf.clone(), Entry { chunks: vec![chunk_no], count: 1 });
                    }
                }
            }
        }
        ChunkIndex { blocks }
    }

    pub fn chunks_with(&self, block: &str) -> Vec<usize> {
        let key = block.to_lowercase();
        self.blocks.get(&key).map(|e| e.chunks.clone()).unwrap_or_default()
    }

    pub fn top_blocks(&self, n: usize) -> Vec<(String, usize)> {
        let mut all: Vec<(&String, usize)> = self.blocks.iter().map(|(b, e)| (b, e.count)).collect();
        all.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
        all.into_iter().take(n).map(|(b, count)| (b.clone(), count)).collect()
    }
}
