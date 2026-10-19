use std::collections::BTreeMap;

/// One line of a mod list: `sodium,fabric,9000,1.2`.
#[derive(Debug, Clone, PartialEq)]
pub struct Mod {
    pub name: String,
    pub loader: String,
    pub downloads: u64,
    pub size_mb: f64,
}

impl Mod {
    /// The jar size in kilobytes: size_mb * 1024. 1.5 MB -> 1536.0
    pub fn size_kb(&self) -> f64 {
        self.size_mb * 1024.0
    }
}

/// One line like "sodium, fabric, 9000, 1.2" -> Mod. Fields are trimmed.
/// Errors: "expected 4 fields, got 2", "bad downloads 'abc'", "bad size_mb 'big'".
pub fn parse_mod(line: &str) -> Result<Mod, String> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();
    if fields.len() != 4 {
        return Err(format!("expected 4 fields, got {}", fields.len()));
    }
    let downloads = fields[2]
        .parse()
        .map_err(|_| format!("bad downloads '{}'", fields[2]))?;
    let size_mb = fields[3]
        .parse()
        .map_err(|_| format!("bad size_mb '{}'", fields[3]))?;
    Ok(Mod {
        name: fields[0].to_string(),
        loader: fields[1].to_string(),
        downloads,
        size_mb,
    })
}

/// The whole CSV text. Line 1 is a header and is skipped; blank lines are skipped.
/// A bad line gives Err("line N: <reason>") where N counts from 1 over ALL lines
/// (header = line 1, blank lines count too).
pub fn parse_modlist(csv: &str) -> Result<Vec<Mod>, String> {
    csv.lines()
        .enumerate()
        .skip(1)
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(i, line)| parse_mod(line).map_err(|e| format!("line {}: {e}", i + 1)))
        .collect()
}

/// Total downloads for each loader, keys sorted by name.
pub fn downloads_by_loader(mods: &[Mod]) -> BTreeMap<String, u64> {
    let mut totals = BTreeMap::new();
    for m in mods {
        *totals.entry(m.loader.clone()).or_insert(0) += m.downloads;
    }
    totals
}

/// The `n` most downloaded mods, biggest first.
pub fn top_n(mods: &[Mod], n: usize) -> Vec<&Mod> {
    let mut sorted: Vec<&Mod> = mods.iter().collect();
    sorted.sort_by_key(|m| std::cmp::Reverse(m.downloads));
    sorted.into_iter().take(n).collect()
}

/// The mod with the lowest size_mb among mods on `loader`. None if there are none.
pub fn smallest_in<'a>(mods: &'a [Mod], loader: &str) -> Option<&'a Mod> {
    mods.iter()
        .filter(|m| m.loader == loader)
        .min_by(|a, b| a.size_mb.total_cmp(&b.size_mb))
}

/// A text table, lines joined with "\n" and no trailing newline:
///   header: format!("{:<10} {:<9} {:>9} {:>7}", "name", "loader", "downloads", "size_mb")
///   per mod: the same widths, size with 1 decimal: {:>7.1}
///   footer: "<count> mods, <sum of downloads> downloads"
pub fn report(mods: &[Mod]) -> String {
    let mut lines = vec![format!(
        "{:<10} {:<9} {:>9} {:>7}",
        "name", "loader", "downloads", "size_mb"
    )];
    lines.extend(mods.iter().map(|m| {
        format!(
            "{:<10} {:<9} {:>9} {:>7.1}",
            m.name, m.loader, m.downloads, m.size_mb
        )
    }));
    let total: u64 = mods.iter().map(|m| m.downloads).sum();
    lines.push(format!("{} mods, {total} downloads", mods.len()));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSV: &str = "name,loader,downloads,size_mb\n\
        sodium,fabric,9000,1.2\n\
        lithium,fabric,7000,0.5\n\
        jei,neoforge,12000,2.0\n\
        create,neoforge,8000,20.0\n";

    fn modded(name: &str, loader: &str, downloads: u64, size_mb: f64) -> Mod {
        Mod {
            name: name.to_string(),
            loader: loader.to_string(),
            downloads,
            size_mb,
        }
    }

    fn sample() -> Vec<Mod> {
        vec![
            modded("sodium", "fabric", 9000, 1.2),
            modded("lithium", "fabric", 7000, 0.5),
            modded("jei", "neoforge", 12000, 2.0),
            modded("create", "neoforge", 8000, 20.0),
        ]
    }

    #[test]
    fn row_total_multiplies_price_by_qty() {
        assert_eq!(modded("jei", "neoforge", 1, 1.5).size_kb(), 1536.0);
        assert_eq!(modded("empty", "fabric", 1, 0.0).size_kb(), 0.0);
    }

    #[test]
    fn parse_line_reads_fields_and_rejects_bad_ones() {
        assert_eq!(
            parse_mod("sodium, fabric ,9000, 1.2"),
            Ok(modded("sodium", "fabric", 9000, 1.2)),
            "fields should be trimmed"
        );
        let e = parse_mod("sodium,fabric").unwrap_err();
        assert!(e.contains("4 fields"), "got: {e}");
        let e = parse_mod("sodium,fabric,lots,1.2").unwrap_err();
        assert!(e.contains("downloads"), "got: {e}");
        let e = parse_mod("sodium,fabric,-3,1.2").unwrap_err();
        assert!(e.contains("downloads"), "got: {e}");
        let e = parse_mod("sodium,fabric,10,big").unwrap_err();
        assert!(e.contains("size"), "got: {e}");
    }

    #[test]
    fn parse_rows_skips_header_and_blanks() {
        assert_eq!(parse_modlist(CSV), Ok(sample()));
        let with_blanks = "name,loader,downloads,size_mb\n\nsodium,fabric,9000,1.2\n   \n";
        assert_eq!(parse_modlist(with_blanks), Ok(vec![modded("sodium", "fabric", 9000, 1.2)]));
        assert_eq!(parse_modlist("name,loader,downloads,size_mb"), Ok(vec![]));
    }

    #[test]
    fn parse_rows_reports_line_number() {
        let bad = "name,loader,downloads,size_mb\n\nsodium,fabric,lots,1\nlithium,fabric,5,1\n";
        let e = parse_modlist(bad).unwrap_err();
        assert!(e.starts_with("line 3:"), "blank lines count; got: {e}");
        assert!(e.contains("downloads"), "got: {e}");
        let short = "name,loader,downloads,size_mb\nsodium,fabric,1,1\njei,neoforge\n";
        let e = parse_modlist(short).unwrap_err();
        assert!(e.starts_with("line 3:"), "got: {e}");
    }

    #[test]
    fn total_by_category_sums_per_category() {
        let totals = downloads_by_loader(&sample());
        assert_eq!(totals.get("fabric"), Some(&16000));
        assert_eq!(totals.get("neoforge"), Some(&20000));
        let keys: Vec<&str> = totals.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, vec!["fabric", "neoforge"], "BTreeMap sorts the keys");
        assert!(downloads_by_loader(&[]).is_empty());
    }

    #[test]
    fn top_n_picks_biggest_totals() {
        let mods = sample();
        let names: Vec<&str> = top_n(&mods, 2).iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["jei", "sodium"]);
        assert_eq!(top_n(&mods, 99).len(), 4, "asking for more than exist is fine");
        assert!(top_n(&mods, 0).is_empty());
    }

    #[test]
    fn cheapest_in_filters_a_category() {
        let mods = sample();
        assert_eq!(smallest_in(&mods, "fabric").map(|m| m.name.as_str()), Some("lithium"));
        assert_eq!(smallest_in(&mods, "neoforge").map(|m| m.name.as_str()), Some("jei"));
        assert_eq!(smallest_in(&mods, "quilt"), None);
    }

    #[test]
    fn report_aligns_columns() {
        let mods = vec![modded("sodium", "fabric", 9000, 1.2), modded("create", "neoforge", 8000, 20.0)];
        let expected = "\
name       loader    downloads size_mb
sodium     fabric         9000     1.2
create     neoforge       8000    20.0
2 mods, 17000 downloads";
        assert_eq!(report(&mods), expected);
        let empty = report(&[]);
        assert!(
            empty.starts_with("name       loader    downloads size_mb\n0 mods"),
            "got: {empty}"
        );
    }
}
