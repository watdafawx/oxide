//! A small text index. It is correct and slow. Keep the public API, make it fast.
//!
//! - `Index::build(text)`: index every word of every line (lines are numbered from 1).
//! - `search(word)`: the lines that contain `word`, ascending, each line once.
//! - `top_words(n)`: the `n` most frequent words as `(word, count)`, most frequent first,
//!   ties in alphabetical order.
//!
//! A word is a run of letters and digits. Matching ignores case: "Rust" and "rust" are one word.

pub mod probe;

use std::collections::HashMap;

struct Entry {
    lines: Vec<usize>,
    count: usize,
}

pub struct Index {
    words: HashMap<String, Entry>,
}

impl Index {
    pub fn build(text: &str) -> Index {
        let mut words: HashMap<String, Entry> = HashMap::new();
        let mut buf = String::new();
        for (i, line) in text.lines().enumerate() {
            let line_no = i + 1;
            for token in line.split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()) {
                buf.clear();
                buf.extend(token.chars().flat_map(char::to_lowercase));
                match words.get_mut(buf.as_str()) {
                    Some(entry) => {
                        entry.count += 1;
                        if entry.lines.last() != Some(&line_no) {
                            entry.lines.push(line_no);
                        }
                    }
                    None => {
                        words.insert(buf.clone(), Entry { lines: vec![line_no], count: 1 });
                    }
                }
            }
        }
        Index { words }
    }

    pub fn search(&self, word: &str) -> Vec<usize> {
        let key = word.to_lowercase();
        self.words.get(&key).map(|e| e.lines.clone()).unwrap_or_default()
    }

    pub fn top_words(&self, n: usize) -> Vec<(String, usize)> {
        let mut all: Vec<(&String, usize)> = self.words.iter().map(|(w, e)| (w, e.count)).collect();
        all.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
        all.into_iter().take(n).map(|(w, count)| (w.clone(), count)).collect()
    }
}
