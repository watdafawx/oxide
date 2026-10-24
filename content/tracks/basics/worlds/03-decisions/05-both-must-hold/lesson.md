The big coaster has two rules: riders must be at least 140 cm tall and at least 10 years old. One comparison cannot say that; two joined together can.

**Both.** `a and b` is true only when `a` is true and `b` is true too. If either one is false, the whole thing is false. `height >= 140 and age >= 10` reads just like the rule on the sign.

**Table it.** With two yes-or-no parts there are only four combinations. For `and`, only "yes and yes" gives yes; every combination with a false in it gives false.

```lua
function M.picnic_weather(temperature, rain)
  local warm = temperature >= 20
  local dry = rain == 0
  return warm and dry        -- a picnic needs both
end
```

**Name the parts.** A long condition reads better split into named booleans, like `warm` and `dry` above. The names say what each rule means, and the last line reads as plain English.

**It stops early.** Lua reads `and` from left to right and stops as soon as the answer is known: if the left side is false, the right side is never looked at. That matters later, when the right side would fail on a missing value.

## Your turn

- In `may_ride`, return true only when `height` is at least 140 and `age` at least 10.
- In `use_fast_lane`, return true only when the visitor `has_pass` and `wait` is over 30.
- Save. The tests try every combination.
