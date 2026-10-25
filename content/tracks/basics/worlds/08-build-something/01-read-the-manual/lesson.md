Games night needs a league table, and its header should be underlined, with team names in capitals. Lua has ready-made functions for both. Programmers find functions like these in the reference manual, and reading it is a skill of its own.

**The entry.** The Lua manual describes every built-in function in a short entry. Here are two of them, word for word as the manual writes them:

```text
string.rep (s, n [, sep])
Returns a string that is the concatenation of n copies of the
string s separated by the string sep. The default value for sep
is the empty string (that is, no separator).

string.upper (s)
Receives a string and returns a copy of this string with all
lowercase letters changed to uppercase.
```

**Read the first line.** The first line is the **signature**: the function's full name and its parameters, in order. `string.rep (s, n [, sep])` takes a string `s`, then a number `n`, and maybe a `sep`. Short names like `s` and `n` are usual in manuals: `s` for a string, `n` for a number.

**Square brackets mean optional.** In docs, `[, sep]` means you may leave that argument out. The text says what happens then: "the default value for sep is the empty string". So `string.rep("ab", 3)` is `"ababab"`, and `string.rep("ab", 3, "-")` is `"ab-ab-ab"`.

**Returns.** "Returns a string" says what the call hands back. "Returns a copy" means the original text is left as it was: `string.upper(name)` does not change `name`, so use what it gives back.

**Try it small.** When an entry is unclear, try one tiny call and look at what comes back. A one-line test answers faster than reading the paragraph a third time. Manuals for Rust and Java look different, but every entry has the same parts: the signature, what each parameter means, and what comes back.

## Your turn

- In `underline`, give back `width` equals signs, using `string.rep`.
- In `banner`, give back `name` in capitals, using `string.upper`.
- In `stars`, give back `n` stars with a space between each, using the optional argument.
