**Why now:** recipes arrive as text lines like `time = 0.5` or `2 iron-plate, 1 copper-cable`. The calculator has to pull them apart. Lua has no regex library: it has its own, smaller **patterns**, and they are all you need.

```lua
local line = "energy = 0.5"
local key, value = line:match("^%s*(%w+)%s*=%s*(.-)%s*$")   -- "energy"  "0.5"

for item in ("a,b,c"):gmatch("[^,]+") do print(item) end   -- a  b  c

local s = ("3 gears"):gsub("%d+", function(n) return n * 2 end)   -- "6 gears"
print(("x.y"):find(".", 1, true))   -- 2 2   (plain find: no pattern magic)
print(("3 + 4"):match("[%+%-%*]"))  -- +      a set [ ] matches ONE of its characters
print(("a,b"):gsub(",", "%%0"))     -- a%0b 1  in a replacement, %% is a literal %, %0 the whole match
```

- Classes: `%d` digit, `%a` letter, `%w` letter or digit, `%s` space, `.` anything.
- Repeat: `+` one or more, `*` zero or more, `-` zero or more, **as few as possible** (lazy), `?` optional.
- A **set** `[abc]` matches one character out of those listed; `[^,]` matches one that is *not* a comma. Inside a set `%` still escapes: `[%+%-]` is plus or minus.
- An upper-case class is the opposite: `%S` is any non-space, `%D` any non-digit.
- `s:match(pattern, i)` starts looking at index `i`; with `^` the match has to begin exactly there.
- In a `gsub` replacement string, `%0` is the whole match and `%%` a literal `%`.
- `( )` captures what is inside. `^` and `$` pin to the start and end.
- `%` is the escape: `%.` is a literal dot. The magic characters are `^$()%.[]*+-?`.
- `gsub` returns the new string **and** a count. Wrap in `( )` to keep just the string.

## Your turn

- Open `src/recipe_text.lua`.
- Fill in `parse_key_value`, `split_commas`, `double_counts` and `escape_pattern`.
