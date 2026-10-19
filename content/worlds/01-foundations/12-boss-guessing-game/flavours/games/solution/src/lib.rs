use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Recipe {
    pub output: String,
    pub yields: u32,
    pub ingredients: Vec<(String, u32)>,
}

#[derive(Default)]
pub struct Calculator {
    recipes: Vec<Recipe>,
}

/// One "name xN" piece: "plank x2" -> Some(("plank", 2)). Zero counts and junk -> None.
/// (Given: you will use it from parse_recipe.)
pub fn parse_part(part: &str) -> Option<(String, u32)> {
    let (name, count) = part.trim().split_once(" x")?;
    let count = count.parse::<u32>().ok()?;
    if name.is_empty() || count == 0 {
        None
    } else {
        Some((name.to_string(), count))
    }
}

/// "need 8 torch" -> Ok((8, "torch")). Anything else -> Err("'...' is not a need request").
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

impl Calculator {
    /// A fresh calculator that knows no recipes yet.
    pub fn new() -> Calculator {
        Calculator {
            recipes: Vec::new(),
        }
    }

    /// How many recipes the calculator knows.
    pub fn recipe_count(&self) -> usize {
        self.recipes.len()
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    /// The recipe that produces `item`, or None for raw materials.
    pub fn find_recipe(&self, item: &str) -> Option<&Recipe> {
        self.recipes.iter().find(|recipe| recipe.output == item)
    }

    /// How many crafts make at least `wanted` of `item`? Raw items need 0 crafts.
    /// With plank yielding 4: batches_needed("plank", 5) -> 2
    pub fn batches_needed(&self, item: &str, wanted: u32) -> u32 {
        match self.find_recipe(item) {
            Some(recipe) => wanted.div_ceil(recipe.yields),
            None => 0,
        }
    }

    /// Raw material totals for `wanted` of `item`, sorted by name.
    pub fn raw_totals(&self, item: &str, wanted: u32) -> BTreeMap<String, u32> {
        let mut totals = BTreeMap::new();
        self.expand(item, wanted, &mut totals);
        totals
    }

    /// Walk the crafting tree. A raw item adds `wanted` to its total. A crafted item
    /// needs `batches_needed` crafts, and each craft needs every ingredient: expand those.
    fn expand(&self, item: &str, wanted: u32, totals: &mut BTreeMap<String, u32>) {
        match self.find_recipe(item) {
            None => *totals.entry(item.to_string()).or_insert(0) += wanted,
            Some(recipe) => {
                let batches = self.batches_needed(item, wanted);
                for (name, count) in &recipe.ingredients {
                    self.expand(name, count * batches, totals);
                }
            }
        }
    }
}

/// "stick x4 = plank x2" -> Ok(Recipe { output: "stick", yields: 4, ingredients: [("plank", 2)] })
/// Any problem -> Err("'<line>' is not a recipe"), with the line trimmed.
pub fn parse_recipe(line: &str) -> Result<Recipe, String> {
    let bad = format!("'{}' is not a recipe", line.trim());
    let (left, right) = match line.split_once('=') {
        Some(sides) => sides,
        None => return Err(bad),
    };
    let (output, yields) = match parse_part(left) {
        Some(pair) => pair,
        None => return Err(bad),
    };
    let mut ingredients = Vec::new();
    for part in right.split(',') {
        match parse_part(part) {
            Some(pair) => ingredients.push(pair),
            None => return Err(bad),
        }
    }
    Ok(Recipe {
        output,
        yields,
        ingredients,
    })
}

/// Drive the calculator from scripted lines and return what the player would see.
///   recipe line  -> "Learned stick" (and the recipe is stored)
///   need 8 stick -> "8 stick needs: log x1" (raw totals, "name xN", joined by ", ")
///   quit         -> "Bye! 1 recipe learned" ("2 recipes" in the plural) and stop
///   anything bad -> its error message; bad lines change nothing
pub fn run(inputs: &[&str]) -> Vec<String> {
    let mut calc = Calculator::new();
    let mut lines = Vec::new();
    for input in inputs {
        let input = input.trim();
        if input == "quit" {
            let n = calc.recipe_count();
            let s = if n == 1 { "" } else { "s" };
            lines.push(format!("Bye! {n} recipe{s} learned"));
            break;
        }
        if input.starts_with("need ") {
            match parse_need(input) {
                Ok((wanted, item)) => {
                    let totals = calc.raw_totals(&item, wanted);
                    let list: Vec<String> = totals
                        .iter()
                        .map(|(name, count)| format!("{name} x{count}"))
                        .collect();
                    lines.push(format!("{wanted} {item} needs: {}", list.join(", ")));
                }
                Err(message) => lines.push(message),
            }
        } else {
            match parse_recipe(input) {
                Ok(recipe) => {
                    lines.push(format!("Learned {}", recipe.output));
                    calc.add_recipe(recipe);
                }
                Err(message) => lines.push(message),
            }
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recipe(output: &str, yields: u32, ingredients: &[(&str, u32)]) -> Recipe {
        Recipe {
            output: output.to_string(),
            yields,
            ingredients: ingredients
                .iter()
                .map(|(name, count)| (name.to_string(), *count))
                .collect(),
        }
    }

    /// torch <- coal + stick, stick <- plank, plank <- log
    fn book() -> Calculator {
        let mut calc = Calculator::default();
        calc.add_recipe(recipe("torch", 4, &[("coal", 1), ("stick", 1)]));
        calc.add_recipe(recipe("stick", 4, &[("plank", 2)]));
        calc.add_recipe(recipe("plank", 4, &[("log", 1)]));
        calc
    }

    fn totals(pairs: &[(&str, u32)]) -> BTreeMap<String, u32> {
        pairs.iter().map(|(name, n)| (name.to_string(), *n)).collect()
    }

    #[test]
    fn calculator_starts_with_no_recipes() {
        let calc = Calculator::new();
        assert_eq!(calc.recipe_count(), 0);
    }

    #[test]
    fn find_recipe_knows_crafted_items() {
        let calc = book();
        assert_eq!(calc.find_recipe("stick").map(|r| r.yields), Some(4));
        assert!(calc.find_recipe("log").is_none(), "logs are raw: no recipe");
    }

    #[test]
    fn batches_needed_rounds_up() {
        let calc = book();
        assert_eq!(calc.batches_needed("plank", 1), 1);
        assert_eq!(calc.batches_needed("plank", 4), 1);
        assert_eq!(calc.batches_needed("plank", 5), 2, "5 planks need a second craft");
        assert_eq!(calc.batches_needed("log", 9), 0, "raw items are not crafted");
    }

    #[test]
    fn raw_totals_walks_the_recipe_tree() {
        let calc = book();
        assert_eq!(
            calc.raw_totals("torch", 8),
            totals(&[("coal", 2), ("log", 1)]),
            "8 torches: 2 crafts, so 2 coal and 2 sticks (1 craft, 2 planks, 1 craft, 1 log)"
        );
        assert_eq!(calc.raw_totals("log", 3), totals(&[("log", 3)]));
        assert_eq!(calc.raw_totals("plank", 5), totals(&[("log", 2)]));
    }

    #[test]
    fn parse_recipe_accepts_a_recipe_line() {
        assert_eq!(
            parse_recipe("torch x4 = coal x1, stick x1"),
            Ok(recipe("torch", 4, &[("coal", 1), ("stick", 1)]))
        );
        assert_eq!(
            parse_recipe("  stick x4 =   plank x2  "),
            Ok(recipe("stick", 4, &[("plank", 2)])),
            "surrounding spaces are fine"
        );
    }

    #[test]
    fn parse_recipe_rejects_nonsense() {
        assert_eq!(parse_recipe("pizza"), Err("'pizza' is not a recipe".to_string()));
        assert!(parse_recipe("stick x0 = plank x1").is_err(), "a recipe yields at least 1");
        assert!(parse_recipe("stick x4 = plank").is_err(), "ingredients need a count");
        assert!(parse_recipe("stick x4 =").is_err(), "no ingredients is not a recipe");
        assert!(parse_recipe("").is_err(), "empty input is not a recipe");
    }

    #[test]
    fn run_turns_console_lines_into_messages() {
        let out = run(&[
            "stick x4 = plank x2",
            "plank x4 = log x1",
            "pizza",
            "need 8 stick",
            "need 1 diamond",
            "need lots stick",
        ]);
        assert_eq!(
            out,
            vec![
                "Learned stick",
                "Learned plank",
                "'pizza' is not a recipe",
                "8 stick needs: log x1",
                "1 diamond needs: diamond x1",
                "'need lots stick' is not a need request",
            ],
            "bad input prints its error and changes nothing"
        );
    }

    #[test]
    fn run_stops_at_quit() {
        let one = run(&["stick x4 = plank x2", "quit", "plank x4 = log x1"]);
        assert_eq!(one, vec!["Learned stick", "Bye! 1 recipe learned"], "singular, and stop");
        let two = run(&["stick x4 = plank x2", "plank x4 = log x1", "quit", "need 1 stick"]);
        assert_eq!(two.last().map(String::as_str), Some("Bye! 2 recipes learned"));
        assert_eq!(two.len(), 3, "nothing runs after quit");
    }
}
