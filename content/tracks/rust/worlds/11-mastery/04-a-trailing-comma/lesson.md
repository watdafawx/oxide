A long word list is easier to read one word per line, and every line, the last one too, ends with a comma: then adding a word at the bottom changes one line, not two. `rustfmt`, the tool that formats Rust code, writes lists that way. `strings!` does not accept it yet, so this starter does not compile.

**Read the error.** The test module calls `strings!` with a comma after the last word, and the compiler stops:

```text
error: unexpected end of macro invocation
   |
   |         "quick",
   |                 ^ missing tokens in macro arguments
   |
note: while trying to match meta-variable `$s:expr`
```

The pattern `$( $s:expr ),*` puts commas *between* expressions. After the last comma it expects another expression, finds the end of the call, and gives up.

**Zero times or once.** Besides `*` and `+` there is a third mark, `?`: the part may appear zero times or once. So `$(,)?` at the end of a pattern is "one comma here, or none". It captures nothing, so the template does not change. `vec!` and `println!` accept a final comma the same way.

## Your turn

- Add an optional comma at the end of the `strings!` pattern.
