Some visitors skip the ticket desk: anyone under 4 gets in free, and so does anyone with a season pass. Either reason is enough, and that needs a different kind of join.

**Either.** `a or b` is true when at least one side is true, and false only when both are false. `age < 4 or has_pass` is the free-entry rule exactly as the sign says it.

**Both counts too.** `or` in code is not "one or the other, but not both". A toddler with a season pass is still free. If you ever need "exactly one of them", that is a different question, built from the pieces you already have.

```lua
function M.is_weekend(day)
  return day == "Saturday" or day == "Sunday"
end
```

**Write both sides out.** `day == "Saturday" or "Sunday"` looks right but is wrong. The right side is just the text `"Sunday"`, not a comparison, and Lua counts any text as yes, so every day becomes the weekend. Each side of `or` must be a full condition.

**Mixing them.** In a line with both `and` and `or`, the `and` is worked out first, the way `*` comes before `+`. Use brackets to say what you mean: `(a or b) and c`.

## Your turn

- In `free_entry`, return true for visitors under 4, or with `has_pass`.
- In `is_closed`, return true before 9 or from 18 on.
- Save. The tests try 9 and 18 exactly.
