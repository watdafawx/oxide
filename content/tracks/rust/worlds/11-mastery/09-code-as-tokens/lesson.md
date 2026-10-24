`macro_rules!` can only rearrange what it is given: match a pattern, paste a template. Some jobs need real code that looks at your code, such as `#[derive(Debug)]`, which reads a struct's fields and writes a `Debug` impl for them. Those are written as Rust functions, and what they receive is not text but tokens.

**The smallest pieces.** Before anything else, the compiler cuts your code into **tokens**, the smallest pieces that mean something: a keyword, a name, a number, a symbol. `let total = 4 * 2;` is seven tokens: `let`, `total`, `=`, `4`, `*`, `2`, `;`. Spaces and comments are not tokens, which is why the compiler does not care how you indent.

**A token stream.** The tokens in a row make a **token stream**. Brackets are special: `( ... )`, `[ ... ]` and `{ ... }` hold their own little stream inside, so a stream nests like the code does. Your `macro_rules!` patterns were matched against exactly this: `$x:expr` meant "the tokens that make one expression".

**A tree with meaning.** A stream does not know that `4 * 2` is a multiplication. The compiler **parses** the tokens into a **syntax tree**: "a `let` that binds `total` to a multiplication of 4 and 2". A tree has names for its parts, so code can ask "what are this struct's fields?" instead of counting commas.

**A function from code to code.** A **procedural macro** (proc macro) is a Rust function the compiler calls while building your program: it gets a token stream and returns a new one. Two crates do the heavy lifting. **syn** parses a stream into a syntax tree you can read, and **quote** turns Rust-looking code you write back into a stream. serde's `#[derive(Serialize)]` and tokio's `#[tokio::main]` are proc macros built this way.

```text
your code ──tokens──▶ the proc macro: syn reads the tree, quote writes new code ──tokens──▶ compiler
```
