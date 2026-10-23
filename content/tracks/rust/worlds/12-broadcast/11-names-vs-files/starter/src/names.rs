//! The asset check: does every player on the roster have a file, and which files belong to nobody?

#![allow(unused_imports)]

use crate::*;

/// A name the way files and sheets can both match it: lower case, common accents dropped, letters and digits only.
pub fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ø' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            other => other,
        })
        .filter(|c| c.is_alphanumeric())
        .collect()
}

#[derive(Debug, Default, PartialEq)]
pub struct NameReport {
    /// Roster names with no file.
    pub missing: Vec<String>,
    /// Files with no roster name.
    pub extra: Vec<String>,
    /// (roster name, file) pairs that are probably the same player, spelled differently.
    pub renames: Vec<(String, String)>,
}

/// Compare roster names with the files in a folder.
pub fn compare_names(names: &[&str], files: &[&str]) -> NameReport {
    let key = |file: &str| normalize_name(file.rsplit_once('.').map_or(file, |(stem, _)| stem));
    let names: Vec<(String, &str)> = names.iter().map(|n| (normalize_name(n), *n)).collect();
    let files: Vec<(String, &str)> = files.iter().map(|f| (key(f), *f)).collect();
    let mut extra: Vec<&(String, &str)> = files.iter().filter(|(k, _)| !names.iter().any(|(n, _)| n == k)).collect();
    let mut report = NameReport::default();
    for (name_key, name) in &names {
        if files.iter().any(|(k, _)| k == name_key) {
            continue;
        }
        match extra.iter().position(|(k, _)| levenshtein(k, name_key) <= 2) {
            Some(i) => report.renames.push((name.to_string(), extra.remove(i).1.to_string())),
            None => report.missing.push(name.to_string()),
        }
    }
    report.extra = extra.into_iter().map(|(_, f)| f.to_string()).collect();
    report
}
