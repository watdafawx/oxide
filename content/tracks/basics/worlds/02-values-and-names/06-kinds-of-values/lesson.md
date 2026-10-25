The ticket machine reads the number of tickets from its keypad as text: `"3"`, not `3`. Mixing the two up is one of the most common bugs there is, and it is the reason programming languages care so much about types.

**Kinds of value.** Every value has a **type**: what kind of thing it is. So far you have met two, numbers (`3`) and strings (`"3"`). On screen they can look exactly alike, but inside the computer they are different things.

**Ask Lua.** `type(v)` tells you the type of a value, as text: `type(3)` is `"number"` and `type("3")` is `"string"`. When something behaves strangely, asking for the type is a good first question.

**Not the same.** `"3" == 3` is `false`: a string never equals a number, however alike they look (`==` asks "are these the same?", and the next world uses it a lot). Lua sometimes converts text for maths, so `"3" + 1` gives 4, but comparing does not convert, and most other languages convert nothing at all.

```lua
local typed = "12"                 -- from a form: text
local seats = tonumber(typed)      -- 12, now a number
local label = tostring(seats)      -- "12", text again
local oops = tonumber("twelve")    -- nil: that text is not a number
```

**Convert on purpose.** `tonumber(text)` turns text into a number, and `tostring(n)` turns a number into text. When the text is not a number, `tonumber` gives `nil`, nothing. Convert once, at the edge where the text comes in, and the rest of the program can work with real numbers.

**Why types exist.** Inside the computer everything is stored as numbers, so a value's type is what says what those numbers mean, and which actions make sense: adding numbers, joining text. Rust and Java check types before the program runs and refuse `"3" + 1` outright; Lua checks while it runs. Either way the idea is the same.

## Your turn

- In `from_keypad`, turn the typed text into a number.
- In `cost_of_typed`, convert first, then charge 9 per ticket.
- Save. The tests also type `"two"`, which is not a number.
