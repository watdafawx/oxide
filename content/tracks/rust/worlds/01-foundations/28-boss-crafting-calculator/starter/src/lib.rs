use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Kit {
    pub output: String,
    pub yields: u32,
    pub ingredients: Vec<(String, u32)>,
}

#[derive(Default)]
pub struct Planner {
    kits: Vec<Kit>,
}

/// One "name xN" piece: "sheet x2" -> Some(("sheet", 2)). Zero counts and junk -> None.
/// (Given: you will use it from parse_kit.)
pub fn parse_part(part: &str) -> Option<(String, u32)> {
    let (name, count) = part.trim().split_once(" x")?;
    let count = count.parse::<u32>().ok()?;
    if name.is_empty() || count == 0 {
        None
    } else {
        Some((name.to_string(), count))
    }
}

/// "need 8 bundle" -> Ok((8, "bundle")). Anything else -> Err("'...' is not a need request").
/// (Given: you will use it from run.)
pub fn parse_need(line: &str) -> Result<(u32, String), String> {
    let line = line.trim();
    let bad = || format!("'{line}' is not a need request");
    let rest = line.strip_prefix("need ").unwrap_or("");
    match rest.split_once(' ') {
        Some((n, item)) if !item.trim().is_empty() => match n.parse::<u32>() {
            Ok(n) => Ok((n, item.trim().to_string())),
            Err(_) => Err(bad()),
        },
        _ => Err(bad()),
    }
}

/// Add `n` to the total for a raw item. (Given: you will use it from expand.)
fn add_raw(totals: &mut BTreeMap<String, u32>, name: &str, n: u32) {
    *totals.entry(name.to_string()).or_insert(0) += n;
}

/// "label x2, crate x1": one line for a whole set of totals. (Given: you will use it from run.)
fn totals_line(totals: &BTreeMap<String, u32>) -> String {
    let parts: Vec<String> = totals.iter().map(|(name, n)| format!("{name} x{n}")).collect();
    parts.join(", ")
}

impl Planner {
    /// A fresh planner that knows no kits yet.
    pub fn new() -> Planner {
        // TODO: build a Planner whose kits are an empty list: `Vec::new()`
        todo!()
    }

    /// How many kits the planner knows.
    pub fn kit_count(&self) -> usize {
        self.kits.len()
    }

    pub fn add_kit(&mut self, kit: Kit) {
        self.kits.push(kit);
    }

    /// The kit that produces `item`, or None for raw materials.
    pub fn find_kit(&self, item: &str) -> Option<&Kit> {
        // TODO: `for kit in &self.kits { ... }`; when `kit.output == item`, `return Some(kit);`
        //       after the loop, the answer is `None`
        todo!()
    }

    /// How many packs make at least `wanted` of `item`? Raw items need 0 packs.
    /// With sheet yielding 4: batches_needed("sheet", 5) -> 2
    pub fn batches_needed(&self, item: &str, wanted: u32) -> u32 {
        // TODO: match self.find_kit(item); a kit gives `wanted.div_ceil(kit.yields)` (divides, rounding up); None gives 0
        todo!()
    }

    /// Raw material totals for `wanted` of `item`, sorted by name.
    pub fn raw_totals(&self, item: &str, wanted: u32) -> BTreeMap<String, u32> {
        let mut totals = BTreeMap::new();
        self.expand(item, wanted, &mut totals);
        totals
    }

    /// Walk the packing tree. A raw item adds `wanted` to its total. A packed item
    /// needs `batches_needed` packs, and each pack needs every ingredient: expand those.
    fn expand(&self, item: &str, wanted: u32, totals: &mut BTreeMap<String, u32>) {
        // TODO: match self.find_kit(item)
        //   None         -> add_raw(totals, item, wanted);
        //   Some(kit) -> let batches = self.batches_needed(item, wanted);
        //                   for (name, count) in &kit.ingredients,
        //                   call self.expand(name, count * batches, totals)
        todo!()
    }
}

/// "box x4 = sheet x2" -> Ok(Kit { output: "box", yields: 4, ingredients: [("sheet", 2)] })
/// Any problem -> Err("'<line>' is not a kit"), with the line trimmed.
pub fn parse_kit(line: &str) -> Result<Kit, String> {
    // TODO: let message = format!("'{}' is not a kit", line.trim());
    //       match line.split_once('=') { Some((left, right)) => ..., None => return Err(message) }
    //       match parse_part(left) gives the output and yields (None => return Err(message))
    //       `let mut ingredients = Vec::new();` then `for piece in right.split(',')`,
    //       match parse_part(piece): Some(part) => ingredients.push(part), None => return Err(message)
    //       finish with Ok(Kit { output, yields, ingredients })
    todo!()
}

/// Drive the planner from scripted lines and return what the user would see.
///   kit line  -> "Learned box" (and the kit is stored)
///   need 8 box -> "8 box needs: crate x1" (raw totals, via totals_line)
///   quit         -> "Bye! 1 kit learned" ("2 kits" in the plural) and stop
///   anything bad -> its error message; bad lines change nothing
pub fn run(inputs: &[&str]) -> Vec<String> {
    // TODO: `let mut calc = Planner::new();` and `let mut messages = Vec::new();`
    //       `for input in inputs`: `let line = input.trim();` then an if chain:
    //         line == "quit"              -> push "Bye! ..." (singular when the count is 1), then `break`
    //         line.starts_with("need ")   -> match parse_need(line): Ok((count, item)) => push
    //                                        "{count} {item} needs: {totals_line(&calc.raw_totals(&item, count))}",
    //                                        Err(message) => push the message
    //         else                        -> match parse_kit(line): Ok(kit) => push "Learned {kit.output}"
    //                                        then calc.add_kit(kit), Err(message) => push the message
    //       finish with `messages`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kit(output: &str, yields: u32, ingredients: &[(&str, u32)]) -> Kit {
        Kit {
            output: output.to_string(),
            yields,
            ingredients: ingredients
                .iter()
                .map(|(name, count)| (name.to_string(), *count))
                .collect(),
        }
    }

    /// bundle <- label + box, box <- sheet, sheet <- crate
    fn book() -> Planner {
        let mut calc = Planner::default();
        calc.add_kit(kit("bundle", 4, &[("label", 1), ("box", 1)]));
        calc.add_kit(kit("box", 4, &[("sheet", 2)]));
        calc.add_kit(kit("sheet", 4, &[("crate", 1)]));
        calc
    }

    fn totals(pairs: &[(&str, u32)]) -> BTreeMap<String, u32> {
        pairs.iter().map(|(name, n)| (name.to_string(), *n)).collect()
    }

    #[test]
    fn planner_starts_with_no_kits() {
        let calc = Planner::new();
        assert_eq!(calc.kit_count(), 0);
    }

    #[test]
    fn find_kit_knows_packed_items() {
        let calc = book();
        assert_eq!(calc.find_kit("box").map(|r| r.yields), Some(4));
        assert!(calc.find_kit("crate").is_none(), "crates are raw: no kit");
    }

    #[test]
    fn batches_needed_rounds_up() {
        let calc = book();
        assert_eq!(calc.batches_needed("sheet", 1), 1);
        assert_eq!(calc.batches_needed("sheet", 4), 1);
        assert_eq!(calc.batches_needed("sheet", 5), 2, "5 sheets need a second pack");
        assert_eq!(calc.batches_needed("crate", 9), 0, "raw items are not packed");
    }

    #[test]
    fn raw_totals_walks_the_kit_tree() {
        let calc = book();
        assert_eq!(
            calc.raw_totals("bundle", 8),
            totals(&[("label", 2), ("crate", 1)]),
            "8 bundles: 2 packs, so 2 label and 2 boxes (1 pack, 2 sheets, 1 pack, 1 crate)"
        );
        assert_eq!(calc.raw_totals("crate", 3), totals(&[("crate", 3)]));
        assert_eq!(calc.raw_totals("sheet", 5), totals(&[("crate", 2)]));
    }

    #[test]
    fn parse_kit_accepts_a_kit_line() {
        assert_eq!(
            parse_kit("bundle x4 = label x1, box x1"),
            Ok(kit("bundle", 4, &[("label", 1), ("box", 1)]))
        );
        assert_eq!(
            parse_kit("  box x4 =   sheet x2  "),
            Ok(kit("box", 4, &[("sheet", 2)])),
            "surrounding spaces are fine"
        );
    }

    #[test]
    fn parse_kit_rejects_nonsense() {
        assert_eq!(parse_kit("pizza"), Err("'pizza' is not a kit".to_string()));
        assert!(parse_kit("box x0 = sheet x1").is_err(), "a kit yields at least 1");
        assert!(parse_kit("box x4 = sheet").is_err(), "ingredients need a count");
        assert!(parse_kit("box x4 =").is_err(), "no ingredients is not a kit");
        assert!(parse_kit("").is_err(), "empty input is not a kit");
    }

    #[test]
    fn run_turns_console_lines_into_messages() {
        let out = run(&[
            "box x4 = sheet x2",
            "sheet x4 = crate x1",
            "pizza",
            "need 8 box",
            "need 1 trophy",
            "need lots box",
        ]);
        assert_eq!(
            out,
            vec![
                "Learned box",
                "Learned sheet",
                "'pizza' is not a kit",
                "8 box needs: crate x1",
                "1 trophy needs: trophy x1",
                "'need lots box' is not a need request",
            ],
            "bad input prints its error and changes nothing"
        );
    }

    #[test]
    fn run_stops_at_quit() {
        let one = run(&["box x4 = sheet x2", "quit", "sheet x4 = crate x1"]);
        assert_eq!(one, vec!["Learned box", "Bye! 1 kit learned"], "singular, and stop");
        let two = run(&["box x4 = sheet x2", "sheet x4 = crate x1", "quit", "need 1 box"]);
        assert_eq!(two.last().map(String::as_str), Some("Bye! 2 kits learned"));
        assert_eq!(two.len(), 3, "nothing runs after quit");
    }
}
