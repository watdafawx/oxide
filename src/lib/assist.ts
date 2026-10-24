// Editor help that needs no language server: completion lists with short notes,
// and plain-English notes for the compiler errors learners hit most.
import { snippetCompletion, type Completion, type CompletionContext, type CompletionResult } from "@codemirror/autocomplete";
import type { EditorView } from "@codemirror/view";
import type { Diagnostic } from "./types";

export type Lang = "rust" | "lua" | "java" | "other";
export const langOfPath = (p: string): Lang =>
  p.endsWith(".rs") ? "rust" : p.endsWith(".lua") ? "lua" : /\.(java|kts?|gradle)$/.test(p) ? "java" : "other";

/** One known name. `insert` uses ${} for the cursor; `eg` is a one-line example. */
interface Entry { label: string; type: Completion["type"]; sig?: string; doc: string; eg?: string; insert?: string; boost?: number }

const R: Entry[] = [
  { label: "fn", type: "keyword", doc: "Starts a function.", insert: "fn ${name}(${}) {\n\t${}\n}" },
  { label: "let", type: "keyword", doc: "Makes a new variable. It can't change unless you write `let mut`.", insert: "let ${name} = ${};" },
  { label: "mut", type: "keyword", doc: "Lets a variable change after it is made: `let mut count = 0;`" },
  { label: "if", type: "keyword", doc: "Runs a block only when the condition is true.", insert: "if ${} {\n\t${}\n}" },
  { label: "else", type: "keyword", doc: "The other branch of an `if`." },
  { label: "match", type: "keyword", doc: "Picks a branch by comparing a value against patterns. Every case must be covered.", insert: "match ${} {\n\t${} => ${},\n}" },
  { label: "for", type: "keyword", doc: "Loops over every item of something you can iterate.", insert: "for ${item} in ${} {\n\t${}\n}" },
  { label: "while", type: "keyword", doc: "Loops while the condition is true." },
  { label: "loop", type: "keyword", doc: "Loops forever, until a `break`." },
  { label: "return", type: "keyword", doc: "Hands a value back early. The last line without `;` already does this." },
  { label: "struct", type: "keyword", doc: "Defines a type with named fields." },
  { label: "enum", type: "keyword", doc: "Defines a type that is one of several variants." },
  { label: "impl", type: "keyword", doc: "Adds methods to a type, or implements a trait for it." },
  { label: "trait", type: "keyword", doc: "A set of methods types can promise to have." },
  { label: "pub", type: "keyword", doc: "Makes an item visible outside its module." },
  { label: "use", type: "keyword", doc: "Brings a name into scope, like `use std::collections::HashMap;`" },
  { label: "self", type: "keyword", doc: "The value a method was called on." },
  { label: "Self", type: "type", doc: "The type you are writing an `impl` for." },
  { label: "format!", type: "function", sig: "format!(\"...{}\", args) -> String", doc: "Builds a String. `{name}` inside the quotes drops in a value.", eg: "format!(\"Hi, {player}!\")", insert: "format!(\"${}\")", boost: 5 },
  { label: "println!", type: "function", sig: "println!(\"...{}\", args)", doc: "Prints a line to the console.", eg: "println!(\"score: {score}\")", insert: "println!(\"${}\");", boost: 4 },
  { label: "vec!", type: "function", sig: "vec![a, b, c] -> Vec<T>", doc: "Makes a Vec (a growable list) from the items.", eg: "vec![1, 2, 3]", insert: "vec![${}]" },
  { label: "assert_eq!", type: "function", sig: "assert_eq!(left, right)", doc: "Fails the test when the two values are different.", insert: "assert_eq!(${}, ${});" },
  { label: "assert!", type: "function", sig: "assert!(condition)", doc: "Fails the test when the condition is false.", insert: "assert!(${});" },
  { label: "panic!", type: "function", sig: "panic!(\"message\")", doc: "Stops the program with a message." },
  { label: "todo!", type: "function", sig: "todo!()", doc: "A placeholder: the code builds, but running it stops with \"not yet implemented\". Replace it." },
  { label: "String", type: "type", doc: "Text you own and can grow. Make one with `String::from(\"hi\")` or `\"hi\".to_string()`." },
  { label: "str", type: "type", doc: "A borrowed piece of text. You mostly see it as `&str`." },
  { label: "Vec", type: "type", doc: "A growable list: `Vec<i32>` holds whole numbers." },
  { label: "Option", type: "type", doc: "Either `Some(value)` or `None`. Rust's way of saying \"maybe missing\"." },
  { label: "Some", type: "function", doc: "The \"there is a value\" side of an Option." },
  { label: "None", type: "constant", doc: "The \"no value\" side of an Option." },
  { label: "Result", type: "type", doc: "Either `Ok(value)` or `Err(error)`, for things that can fail." },
  { label: "Ok", type: "function", doc: "The success side of a Result." },
  { label: "Err", type: "function", doc: "The failure side of a Result." },
  { label: "HashMap", type: "type", doc: "A lookup table from keys to values. `use std::collections::HashMap;`" },
  { label: "i32", type: "type", doc: "A whole number, positive or negative." },
  { label: "u32", type: "type", doc: "A whole number, zero or more." },
  { label: "usize", type: "type", doc: "A whole number used for sizes and indexes." },
  { label: "f64", type: "type", doc: "A number with a decimal point." },
  { label: "bool", type: "type", doc: "`true` or `false`." },
];
const RM: Entry[] = [
  { label: "to_string", type: "method", sig: "fn to_string(&self) -> String", doc: "Makes an owned String from this value.", eg: "\"hi\".to_string()", insert: "to_string()", boost: 3 },
  { label: "to_uppercase", type: "method", sig: "fn to_uppercase(&self) -> String", doc: "A new String with every letter in capitals.", eg: "\"Ferris\".to_uppercase() // \"FERRIS\"", insert: "to_uppercase()" },
  { label: "to_lowercase", type: "method", sig: "fn to_lowercase(&self) -> String", doc: "A new String with every letter in small letters.", insert: "to_lowercase()" },
  { label: "to_owned", type: "method", sig: "fn to_owned(&self) -> String", doc: "An owned copy (for &str that is a String).", insert: "to_owned()" },
  { label: "trim", type: "method", sig: "fn trim(&self) -> &str", doc: "The text without spaces at the start and end.", eg: "\"  hi \".trim() // \"hi\"", insert: "trim()" },
  { label: "len", type: "method", sig: "fn len(&self) -> usize", doc: "How many items (or bytes, for text).", insert: "len()", boost: 2 },
  { label: "is_empty", type: "method", sig: "fn is_empty(&self) -> bool", doc: "True when there is nothing in it.", insert: "is_empty()" },
  { label: "push", type: "method", sig: "fn push(&mut self, value: T)", doc: "Adds an item to the end of a Vec. The Vec must be `mut`.", insert: "push(${})" },
  { label: "push_str", type: "method", sig: "fn push_str(&mut self, s: &str)", doc: "Adds text to the end of a String. The String must be `mut`.", insert: "push_str(${})" },
  { label: "pop", type: "method", sig: "fn pop(&mut self) -> Option<T>", doc: "Removes and returns the last item, or None when empty.", insert: "pop()" },
  { label: "contains", type: "method", sig: "fn contains(&self, x) -> bool", doc: "True when the value is in there.", insert: "contains(${})" },
  { label: "starts_with", type: "method", sig: "fn starts_with(&self, p: &str) -> bool", doc: "True when the text begins with `p`.", insert: "starts_with(${})" },
  { label: "split", type: "method", sig: "fn split(&self, sep) -> Split", doc: "Cuts text into pieces at each `sep`.", eg: "\"a,b\".split(',')", insert: "split(${})" },
  { label: "chars", type: "method", sig: "fn chars(&self) -> Chars", doc: "Goes through the text one character at a time.", insert: "chars()" },
  { label: "iter", type: "method", sig: "fn iter(&self) -> Iter", doc: "Goes through the items, borrowing each one.", insert: "iter()", boost: 2 },
  { label: "into_iter", type: "method", sig: "fn into_iter(self) -> IntoIter", doc: "Goes through the items, taking ownership of them.", insert: "into_iter()" },
  { label: "map", type: "method", sig: "fn map(self, f) -> Map", doc: "Changes every item with a closure.", eg: "nums.iter().map(|n| n * 2)", insert: "map(|${x}| ${})" },
  { label: "filter", type: "method", sig: "fn filter(self, f) -> Filter", doc: "Keeps only the items where the closure returns true.", insert: "filter(|${x}| ${})" },
  { label: "collect", type: "method", sig: "fn collect<B>(self) -> B", doc: "Gathers iterator items into a collection, like a Vec or String.", eg: "let v: Vec<_> = it.collect();", insert: "collect()" },
  { label: "sum", type: "method", sig: "fn sum<S>(self) -> S", doc: "Adds up all the items.", insert: "sum()" },
  { label: "count", type: "method", sig: "fn count(self) -> usize", doc: "How many items the iterator has.", insert: "count()" },
  { label: "enumerate", type: "method", sig: "fn enumerate(self) -> Enumerate", doc: "Pairs each item with its index: (0, a), (1, b)...", insert: "enumerate()" },
  { label: "unwrap", type: "method", sig: "fn unwrap(self) -> T", doc: "Takes the value out of an Option/Result, and stops the program if it is None/Err.", insert: "unwrap()" },
  { label: "unwrap_or", type: "method", sig: "fn unwrap_or(self, default: T) -> T", doc: "The value, or `default` when it is None/Err.", insert: "unwrap_or(${})" },
  { label: "expect", type: "method", sig: "fn expect(self, msg: &str) -> T", doc: "Like unwrap, with your own message when it fails.", insert: "expect(\"${}\")" },
  { label: "clone", type: "method", sig: "fn clone(&self) -> Self", doc: "Makes a full copy, so you keep the original.", insert: "clone()" },
  { label: "get", type: "method", sig: "fn get(&self, key) -> Option<&V>", doc: "Looks up an item. Gives None instead of crashing when it is missing.", insert: "get(${})" },
  { label: "insert", type: "method", sig: "fn insert(&mut self, k, v)", doc: "Puts a key and value in a HashMap.", insert: "insert(${}, ${})" },
  { label: "is_some", type: "method", sig: "fn is_some(&self) -> bool", doc: "True when the Option holds a value.", insert: "is_some()" },
  { label: "is_none", type: "method", sig: "fn is_none(&self) -> bool", doc: "True when the Option is None.", insert: "is_none()" },
  { label: "as_str", type: "method", sig: "fn as_str(&self) -> &str", doc: "Borrows a String as &str.", insert: "as_str()" },
  { label: "join", type: "method", sig: "fn join(&self, sep: &str) -> String", doc: "Glues a list of text together with `sep` between items.", insert: "join(${})" },
  { label: "rev", type: "method", sig: "fn rev(self) -> Rev", doc: "Goes through the items backwards.", insert: "rev()" },
  { label: "max", type: "method", sig: "fn max(self) -> Option<T>", doc: "The biggest item (None when empty).", insert: "max()" },
  { label: "min", type: "method", sig: "fn min(self) -> Option<T>", doc: "The smallest item (None when empty).", insert: "min()" },
];

const L: Entry[] = [
  { label: "local", type: "keyword", doc: "Makes a variable that only exists in this block or file.", insert: "local ${name} = ${}" },
  { label: "function", type: "keyword", doc: "Starts a function. Close it with `end`.", insert: "function ${name}(${})\n\t${}\nend" },
  { label: "end", type: "keyword", doc: "Closes a function, if, for or while block." },
  { label: "if", type: "keyword", doc: "Runs a block when the condition is true: `if x then ... end`.", insert: "if ${} then\n\t${}\nend" },
  { label: "then", type: "keyword", doc: "Comes after an `if` condition." },
  { label: "elseif", type: "keyword", doc: "Another condition to try after `if`." },
  { label: "else", type: "keyword", doc: "The other branch of an `if`." },
  { label: "for", type: "keyword", doc: "Loops. `for i = 1, 10 do ... end` or `for k, v in pairs(t) do ... end`.", insert: "for ${i} = 1, ${n} do\n\t${}\nend" },
  { label: "while", type: "keyword", doc: "Loops while the condition is true.", insert: "while ${} do\n\t${}\nend" },
  { label: "return", type: "keyword", doc: "Hands a value back from a function." },
  { label: "nil", type: "constant", doc: "\"Nothing here.\" Using nil where a value is needed is the most common Lua error." },
  { label: "true", type: "constant", doc: "Yes." }, { label: "false", type: "constant", doc: "No." },
  { label: "and", type: "keyword", doc: "Both sides must be true." }, { label: "or", type: "keyword", doc: "Either side is true. Also gives a default: `name or \"guest\"`." },
  { label: "not", type: "keyword", doc: "Flips true and false." },
  { label: "print", type: "function", sig: "print(...)", doc: "Writes values to the console.", insert: "print(${})" },
  { label: "pairs", type: "function", sig: "pairs(t)", doc: "Goes through every key and value of a table.", insert: "pairs(${})" },
  { label: "ipairs", type: "function", sig: "ipairs(t)", doc: "Goes through a list in order: 1, 2, 3...", insert: "ipairs(${})" },
  { label: "tostring", type: "function", sig: "tostring(v) -> string", doc: "Turns any value into text.", insert: "tostring(${})" },
  { label: "tonumber", type: "function", sig: "tonumber(s) -> number|nil", doc: "Turns text into a number, or nil when it isn't one.", insert: "tonumber(${})" },
  { label: "type", type: "function", sig: "type(v) -> string", doc: "The kind of a value: \"number\", \"string\", \"table\", \"nil\"...", insert: "type(${})" },
  { label: "string", type: "namespace", doc: "Text helpers: string.format, string.upper, string.sub..." },
  { label: "table", type: "namespace", doc: "List helpers: table.insert, table.remove, table.concat..." },
  { label: "math", type: "namespace", doc: "Number helpers: math.floor, math.max, math.random..." },
];
const LM: Record<string, Entry[]> = {
  string: [
    { label: "format", type: "function", sig: "string.format(fmt, ...)", doc: "Builds text. %s drops in text, %d a whole number.", eg: "string.format(\"Hi, %s!\", name)", insert: "format(\"${}\", ${})" },
    { label: "upper", type: "function", sig: "string.upper(s)", doc: "Text in capitals.", insert: "upper(${})" },
    { label: "lower", type: "function", sig: "string.lower(s)", doc: "Text in small letters.", insert: "lower(${})" },
    { label: "len", type: "function", sig: "string.len(s)", doc: "How many characters. `#s` does the same.", insert: "len(${})" },
    { label: "sub", type: "function", sig: "string.sub(s, i, j)", doc: "Part of the text, from position i to j (starting at 1).", insert: "sub(${}, ${})" },
    { label: "rep", type: "function", sig: "string.rep(s, n)", doc: "The text repeated n times.", insert: "rep(${}, ${})" },
    { label: "find", type: "function", sig: "string.find(s, pattern)", doc: "Where the pattern is, or nil.", insert: "find(${}, ${})" },
    { label: "gsub", type: "function", sig: "string.gsub(s, pattern, repl)", doc: "Replaces every match.", insert: "gsub(${}, ${}, ${})" },
  ],
  table: [
    { label: "insert", type: "function", sig: "table.insert(t, v)", doc: "Adds v to the end of the list.", insert: "insert(${}, ${})" },
    { label: "remove", type: "function", sig: "table.remove(t, i)", doc: "Takes out the item at i (the last one when i is left out).", insert: "remove(${})" },
    { label: "concat", type: "function", sig: "table.concat(t, sep)", doc: "Joins a list of text with sep between items.", insert: "concat(${}, ${})" },
    { label: "sort", type: "function", sig: "table.sort(t)", doc: "Sorts the list in place.", insert: "sort(${})" },
  ],
  math: [
    { label: "floor", type: "function", sig: "math.floor(x)", doc: "Rounds down to a whole number.", insert: "floor(${})" },
    { label: "ceil", type: "function", sig: "math.ceil(x)", doc: "Rounds up to a whole number.", insert: "ceil(${})" },
    { label: "max", type: "function", sig: "math.max(a, b, ...)", doc: "The biggest of the numbers.", insert: "max(${})" },
    { label: "min", type: "function", sig: "math.min(a, b, ...)", doc: "The smallest of the numbers.", insert: "min(${})" },
    { label: "random", type: "function", sig: "math.random(a, b)", doc: "A random whole number from a to b.", insert: "random(${})" },
    { label: "abs", type: "function", sig: "math.abs(x)", doc: "The number without its minus sign.", insert: "abs(${})" },
  ],
};
// `s:upper()` style calls on a string value
const LS: Entry[] = LM.string.map((e) => ({ ...e, type: "method", insert: e.insert?.replace(/^(\w+)\(\$\{\}(, )?/, "$1(") }));

const J: Entry[] = [
  { label: "public", type: "keyword", doc: "Visible from everywhere." },
  { label: "private", type: "keyword", doc: "Visible only inside this class." },
  { label: "static", type: "keyword", doc: "Belongs to the class itself, not to one object." },
  { label: "final", type: "keyword", doc: "Can't change after it is set." },
  { label: "class", type: "keyword", doc: "Defines a class." },
  { label: "record", type: "keyword", doc: "A small class that just holds values: `record Point(int x, int y) {}`" },
  { label: "return", type: "keyword", doc: "Hands a value back from the method." },
  { label: "new", type: "keyword", doc: "Makes a new object: `new ArrayList<>()`." },
  { label: "if", type: "keyword", doc: "Runs a block when the condition is true.", insert: "if (${}) {\n\t${}\n}" },
  { label: "for", type: "keyword", doc: "Loops: `for (var item : list) { ... }`.", insert: "for (var ${item} : ${}) {\n\t${}\n}" },
  { label: "var", type: "keyword", doc: "Lets Java work out the variable's type: `var name = \"Steve\";`" },
  { label: "String", type: "type", doc: "Text. Join pieces with + or String.format." },
  { label: "int", type: "type", doc: "A whole number." }, { label: "boolean", type: "type", doc: "true or false." }, { label: "double", type: "type", doc: "A number with a decimal point." },
  { label: "List", type: "type", doc: "An ordered list. `List.of(a, b)` or `new ArrayList<>()`." },
  { label: "Map", type: "type", doc: "Keys to values. `new HashMap<>()`." },
  { label: "System.out.println", type: "function", sig: "System.out.println(x)", doc: "Prints a line to the console.", insert: "System.out.println(${});" },
  { label: "String.format", type: "function", sig: "String.format(fmt, args...)", doc: "Builds text. %s drops in a value, %d a whole number.", insert: "String.format(\"${}\", ${})" },
];
const JM: Entry[] = [
  { label: "length", type: "method", sig: "int length()", doc: "How many characters the text has.", insert: "length()" },
  { label: "toUpperCase", type: "method", sig: "String toUpperCase()", doc: "The text in capitals.", insert: "toUpperCase()" },
  { label: "toLowerCase", type: "method", sig: "String toLowerCase()", doc: "The text in small letters.", insert: "toLowerCase()" },
  { label: "equals", type: "method", sig: "boolean equals(Object o)", doc: "Compares contents. Use this for Strings, not ==.", insert: "equals(${})" },
  { label: "isEmpty", type: "method", sig: "boolean isEmpty()", doc: "True when there is nothing in it.", insert: "isEmpty()" },
  { label: "trim", type: "method", sig: "String trim()", doc: "The text without spaces at both ends.", insert: "trim()" },
  { label: "contains", type: "method", sig: "boolean contains(x)", doc: "True when x is in there.", insert: "contains(${})" },
  { label: "add", type: "method", sig: "boolean add(E e)", doc: "Adds an item to the list.", insert: "add(${})" },
  { label: "get", type: "method", sig: "E get(int i) / V get(K k)", doc: "Reads an item by index (List) or key (Map).", insert: "get(${})" },
  { label: "put", type: "method", sig: "V put(K k, V v)", doc: "Stores a value under a key in a Map.", insert: "put(${}, ${})" },
  { label: "size", type: "method", sig: "int size()", doc: "How many items.", insert: "size()" },
  { label: "stream", type: "method", sig: "Stream<E> stream()", doc: "Starts a chain like .filter(...).map(...).toList().", insert: "stream()" },
  { label: "toList", type: "method", sig: "List<T> toList()", doc: "Collects a stream into a List.", insert: "toList()" },
  { label: "charAt", type: "method", sig: "char charAt(int i)", doc: "The character at position i (from 0).", insert: "charAt(${})" },
  { label: "substring", type: "method", sig: "String substring(int a, int b)", doc: "Part of the text from a up to (not including) b.", insert: "substring(${})" },
];

const DICT: Record<Lang, { top: Entry[]; members: Entry[] }> = {
  rust: { top: R, members: RM },
  lua: { top: L, members: LS },
  java: { top: J, members: JM },
  other: { top: [], members: [] },
};

function infoDom(e: Entry): () => Node {
  return () => {
    const box = document.createElement("div");
    box.className = "ox-info";
    if (e.sig) { const c = document.createElement("code"); c.className = "ox-sig"; c.textContent = e.sig; box.append(c); }
    const p = document.createElement("p");
    rich(p, e.doc);
    box.append(p);
    if (e.eg) { const c = document.createElement("div"); c.className = "ox-eg"; c.textContent = e.eg; box.append(c); }
    return box;
  };
}
const toCompletion = (e: Entry): Completion => {
  const base = { label: e.label, type: e.type, detail: e.sig ? e.sig.replace(/^.*?(->|:)\s*/, "→ ").slice(0, 28) : undefined, info: infoDom(e), boost: e.boost ?? 0 };
  return e.insert ? snippetCompletion(e.insert, base) : base;
};

/** Every word already in the file (names you made yourself). */
function localWords(ctx: CompletionContext, skip: string): Completion[] {
  const text = ctx.state.doc.toString();
  const seen = new Set<string>();
  for (const m of text.matchAll(/[A-Za-z_][A-Za-z0-9_]{2,}/g)) if (m[0] !== skip) seen.add(m[0]);
  return [...seen].map((w) => ({ label: w, type: "variable", boost: -1 }));
}

export function completionSource(lang: Lang) {
  const d = DICT[lang];
  return (ctx: CompletionContext): CompletionResult | null => {
    const line = ctx.state.doc.lineAt(ctx.pos);
    const before = line.text.slice(0, ctx.pos - line.from);
    // inside a comment: stay quiet
    if (/(\/\/|--|#)/.test(before.replace(/"[^"]*"/g, "")) && /^\s*(\/\/|--|#)/.test(before)) return null;
    const member = /([A-Za-z_][\w]*|\)|\])\s*([.:])([A-Za-z_]\w*)?$/.exec(before);
    if (member) {
      const from = ctx.pos - (member[3]?.length ?? 0);
      const lib = lang === "lua" && member[2] === "." ? LM[member[1]] : undefined;
      const list = lib ?? d.members;
      if (!list.length) return null;
      return { from, options: list.map(toCompletion), validFor: /^\w*$/ };
    }
    const w = ctx.matchBefore(/[A-Za-z_][\w!]*/);
    if (!w || (w.from === w.to && !ctx.explicit)) return null;
    if (!ctx.explicit && w.to - w.from < 2) return null;
    const typed = ctx.state.sliceDoc(w.from, w.to);
    const known = new Set(d.top.map((e) => e.label));
    return {
      from: w.from,
      options: [...d.top.map(toCompletion), ...localWords(ctx, typed).filter((c) => !known.has(c.label))],
      validFor: /^[\w!]*$/,
    };
  };
}

/** short note for a word under the mouse, or null */
export function lookup(lang: Lang, word: string, member: boolean): Entry | null {
  const d = DICT[lang];
  const pool = member ? [...d.members, ...d.top] : [...d.top, ...d.members];
  return pool.find((e) => e.label === word || e.label === word + "!") ?? null;
}
export const docDom = (e: Entry) => infoDom(e)() as HTMLElement;

/** put `code` spans in place of backticks; never uses innerHTML */
export function rich(el: HTMLElement, text: string) {
  text.split(/(`[^`]+`)/).forEach((part) => {
    if (part.startsWith("`") && part.endsWith("`") && part.length > 2) {
      const c = document.createElement("code");
      c.textContent = part.slice(1, -1);
      el.append(c);
    } else el.append(part);
  });
}

// ---------------------------------------------------------------- errors

export interface Fix { label: string; apply: (view: EditorView) => boolean }
export interface Explained { title: string; body: string; tip?: string; fix?: Fix }

const tick = (s: string | undefined) => (s ? `\`${s}\`` : "it");

/** remove the `;` on the last statement of the function the error points into */
function dropTrailingSemicolon(lineNo: number): Fix {
  return {
    label: "Remove the ;",
    apply(view) {
      const doc = view.state.doc;
      if (lineNo > doc.lines) return false;
      const indent = (s: string) => s.length - s.trimStart().length;
      const startIndent = indent(doc.line(lineNo).text);
      // find the closing brace of the block that starts at or contains this line
      let close = -1;
      for (let n = lineNo + 1; n <= doc.lines; n++) {
        const t = doc.line(n).text;
        if (t.trim() === "}" && indent(t) <= startIndent) { close = n; break; }
      }
      if (close < 0) return false;
      for (let n = close - 1; n > lineNo - 1 && n >= 1; n--) {
        const l = doc.line(n);
        const t = l.text.trimEnd();
        if (!t.trim()) continue;
        if (!t.endsWith(";")) return false;
        const at = l.from + t.length - 1;
        view.dispatch({ changes: { from: at, to: at + 1 }, selection: { anchor: at } });
        return true;
      }
      return false;
    },
  };
}

function addMut(name: string): Fix {
  return {
    label: `Make ${name} changeable (add mut)`,
    apply(view) {
      const doc = view.state.doc;
      const re = new RegExp(`\\blet\\s+(${name.replace(/[^\w]/g, "")})\\b`);
      for (let n = 1; n <= doc.lines; n++) {
        const l = doc.line(n);
        const m = re.exec(l.text);
        if (!m) continue;
        const at = l.from + m.index + m[0].length - m[1].length;
        view.dispatch({ changes: { from: at, insert: "mut " } });
        return true;
      }
      return false;
    },
  };
}

/** Explain one diagnostic in plain words. Always returns something; unknown errors keep the compiler's own message. */
export function explain(d: Diagnostic, lang: Lang): Explained {
  const all = `${d.message}\n${d.rendered ?? ""}`;
  const name = (re: RegExp) => re.exec(all)?.[1];
  const help = /^\s*(?:=\s*)?help: (.+)$/m.exec(d.rendered ?? "")?.[1];
  const tip = help ? `The compiler suggests: ${help}` : undefined;

  if (lang === "rust") {
    const code = /error\[(E\d{4})\]/.exec(all)?.[1];
    const exp = /expected `([^`]+)`, found `([^`]+)`/.exec(all);
    if ((exp && exp[2] === "()") || /remove this semicolon/.test(all))
      return {
        title: "This function hands back nothing",
        body: "The last line ends with `;`, which turns it into a statement. So the function returns `()` (nothing) instead of the value it promised. Drop the `;` to hand the value back.",
        fix: d.line ? dropTrailingSemicolon(d.line) : undefined,
      };
    if (code === "E0308" || /mismatched types/.test(all))
      return { title: "Wrong kind of value", body: exp ? `Rust expected ${tick(exp[1])} here, but this is ${tick(exp[2])}.` : "This value has a different type than the code around it expects.", tip };
    if (code === "E0425" || /cannot find value/.test(all)) {
      const n = name(/cannot find value `([^`]+)`/);
      return { title: `Rust doesn't know ${tick(n)}`, body: `There is no variable called ${tick(n)} here. Check the spelling, or make it first with \`let\`. Names only exist inside the { } block they were made in.`, tip };
    }
    if (code === "E0384" || /cannot assign twice to immutable/.test(all)) {
      const n = name(/immutable variable `([^`]+)`/);
      return { title: `${tick(n)} can't change`, body: "Variables in Rust are fixed once set. Write `let mut` when you make it, so it can change later.", fix: n ? addMut(n) : undefined };
    }
    if (code === "E0596" || /cannot borrow .* as mutable, as it is not declared as mutable/.test(all)) {
      const n = name(/cannot borrow `([^`]+)` as mutable/);
      return { title: `${tick(n)} needs to be mut`, body: "You are changing this value (push, insert...), but it wasn't made with `let mut`.", fix: n ? addMut(n) : undefined };
    }
    if (code === "E0382" || /use of moved value|borrow of moved value/.test(all)) {
      const n = name(/moved value: `([^`]+)`/);
      return { title: `${tick(n)} was already given away`, body: `Passing ${tick(n)} to a function or another variable moved it there, so it can't be used here anymore. Pass a borrow (\`&${n ?? "x"}\`) instead, or \`.clone()\` it if you need two copies.`, tip };
    }
    if (code === "E0502" || code === "E0499")
      return { title: "Two borrows clash", body: "Something is reading this value while something else changes it. Finish using the first borrow before you change the value.", tip };
    if (code === "E0599" || /no method named/.test(all)) {
      const n = name(/no method named `([^`]+)`/);
      return { title: `No method ${tick(n)} here`, body: `This value doesn't have a ${tick(n)} method. Check the spelling, or type \`.\` and press Ctrl+Space to see what it does have.`, tip };
    }
    if (code === "E0061" || /this function takes \d+ argument/.test(all)) {
      const m = /takes (\d+) arguments? but (\d+) arguments? (?:was|were) supplied/.exec(all);
      return { title: "Wrong number of inputs", body: m ? `This function wants ${m[1]}, but got ${m[2]}.` : "This call passes a different number of values than the function takes.", tip };
    }
    if (code === "E0277" && /doesn't implement `std::fmt::Display`|cannot be formatted with the default formatter/.test(all))
      return { title: "Can't print this directly", body: "`{}` only works for things with a normal text form. Use `{:?}` to print the debug view instead.", tip };
    if (code === "E0277") return { title: "This type can't do that", body: "The value is missing a trait this code needs (like being added, compared or printed).", tip };
    if (code === "E0433" || /failed to resolve|use of undeclared type/.test(all)) {
      const n = name(/undeclared type `([^`]+)`/) ?? name(/`([^`]+)`/);
      return { title: `Rust can't find ${tick(n)}`, body: "It may need a `use` line at the top, like `use std::collections::HashMap;`, or the name is misspelled.", tip };
    }
    if (code === "E0369") return { title: "That operator doesn't work here", body: "You can't use this +, -, == (etc.) between these two types. Convert one side first.", tip };
    if (code === "E0106") return { title: "Rust needs to know how long this borrow lives", body: "A returned `&` needs a lifetime, like `<'a>`. Often it is easier to return an owned value (a `String`) instead.", tip };
    if (/expected `;`|expected one of .*`;`/.test(all)) return { title: "Missing ;", body: "Rust expected a `;` to end the line before this point.", tip };
    if (/unclosed delimiter|mismatched closing delimiter|this file contains an unclosed/.test(all)) return { title: "A bracket isn't closed", body: "Some `(`, `[` or `{` has no partner. Count them around this line.", tip };
    if (/unused variable/.test(all)) {
      const n = name(/unused variable: `([^`]+)`/);
      return { title: `${tick(n)} is never used`, body: `Only a warning. Use it, or name it \`_${n ?? "x"}\` to say \"on purpose\".` };
    }
    if (/variable does not need to be mutable/.test(all)) return { title: "This mut isn't needed", body: "Only a warning: the variable never changes, so `mut` can go." };
  }

  if (lang === "lua") {
    const local = name(/\((?:local|global|field|upvalue) '([^']+)'\)/);
    if (/attempt to call a nil value/.test(all)) return { title: `${tick(local)} isn't a function`, body: `Lua tried to call ${tick(local)}, but it is nil. Check the spelling, and that the function is defined above this line.` };
    if (/attempt to index a nil value/.test(all)) return { title: `${tick(local)} is nil`, body: `You used \`.\` or \`[ ]\` on ${tick(local)}, but it holds nothing. Make sure the table exists first.` };
    if (/attempt to concatenate a nil value/.test(all)) return { title: `${tick(local)} is nil`, body: `\`..\` joins text, but ${tick(local)} holds nothing. Give it a value, or use a default: \`(${local ?? "x"} or "")\`.` };
    if (/attempt to perform arithmetic on/.test(all)) return { title: "Math on something that isn't a number", body: `${tick(local)} is not a number. Use \`tonumber()\` on text first.` };
    if (/attempt to compare/.test(all)) return { title: "Comparing different kinds", body: "Lua can't compare these two values with < or >. Make them both numbers (or both text)." };
    if (/'end' expected/.test(all)) return { title: "Missing end", body: "A `function`, `if`, `for` or `while` block isn't closed. Add `end` where it should stop." };
    if (/'then' expected/.test(all)) return { title: "Missing then", body: "Lua writes `if condition then ... end`." };
    if (/'do' expected/.test(all)) return { title: "Missing do", body: "Loops are `for ... do ... end` and `while ... do ... end`." };
    if (/unexpected symbol|'=' expected/.test(all)) return { title: "Lua can't read this line", body: "There is a typo near here: a missing comma, quote, or bracket." };
  }

  if (lang === "java") {
    if (/cannot find symbol/.test(all)) {
      const n = name(/symbol:\s+\w+\s+(\w+)/);
      return { title: `Java doesn't know ${tick(n)}`, body: "Check the spelling and capitals (Java cares about both), and that it is declared or imported.", tip };
    }
    if (/';' expected/.test(all)) return { title: "Missing ;", body: "Java statements end with `;`." };
    if (/incompatible types/.test(all)) {
      const m = /incompatible types: (.+?) cannot be converted to (.+)/.exec(all);
      return { title: "Wrong kind of value", body: m ? `This is ${tick(m[1])}, but ${tick(m[2])} is needed here.` : "This value's type doesn't fit here.", tip };
    }
    if (/missing return statement/.test(all)) return { title: "The method doesn't always return", body: "Every path through the method needs a `return` with a value." };
    if (/might not have been initialized/.test(all)) return { title: "Variable may be empty", body: "Give the variable a value before you read it." };
    if (/unreachable statement/.test(all)) return { title: "This line can never run", body: "It comes after a `return` (or `break`), so Java can't get here." };
    if (/should be declared in a file named/.test(all)) return { title: "File and class name differ", body: "A public class must live in a file with the same name." };
  }

  return { title: d.level === "error" ? "The code doesn't build yet" : "A warning", body: d.message, tip };
}
