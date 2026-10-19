//! A small text index. It is correct and slow. Keep the public API, make it fast.
//!
//! - `Index::build(text)`: index every word of every line (lines are numbered from 1).
//! - `search(word)`: the lines that contain `word`, ascending, each line once.
//! - `top_words(n)`: the `n` most frequent words as `(word, count)`, most frequent first,
//!   ties in alphabetical order.
//!
//! A word is a run of letters and digits. Matching ignores case: "Rust" and "rust" are one word.

pub mod probe;

use probe::Word;

pub struct Index {
    // One (word, line) pair per occurrence, in order of first appearance.
    entries: Vec<(Word, Vec<usize>)>,
}

impl Index {
    pub fn build(text: &str) -> Index {
        // TODO: for every token this allocates a lowercase String and scans every known word.
        // Look the word up in a `HashMap` instead, and reuse one buffer for lowercasing.
        let mut entries: Vec<(Word, Vec<usize>)> = Vec::new();
        for (i, line) in text.lines().enumerate() {
            let line_no = i + 1;
            for token in line.split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()) {
                let word = Word(token.to_lowercase());
                match entries.iter_mut().find(|(w, _)| *w == word) {
                    Some((_, lines)) => lines.push(line_no),
                    None => entries.push((word, vec![line_no])),
                }
            }
        }
        Index { entries }
    }

    pub fn search(&self, word: &str) -> Vec<usize> {
        // TODO: this walks every known word. With a HashMap it is one lookup.
        let word = Word(word.to_lowercase());
        for (w, lines) in &self.entries {
            if *w == word {
                let mut out = lines.clone();
                out.dedup();
                return out;
            }
        }
        Vec::new()
    }

    pub fn top_words(&self, n: usize) -> Vec<(String, usize)> {
        // TODO: this clones every word. Sort references, clone only the `n` you return.
        let mut all: Vec<(Word, usize)> =
            self.entries.iter().map(|(w, lines)| (w.clone(), lines.len())).collect();
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
        all.into_iter().map(|(w, count)| (w.0, count)).collect()
    }
}
