The welcome screen needs two lines: a greeting, then the opening hours. Your first program prints them, in that order.

**Show some text.** `print("Hi")` shows the text `Hi` on the screen. `print` is a ready-made instruction that comes with Lua, and the round brackets hold what it should show.

**Quotes mark text.** The quotes tell Lua "this is text, show it as it is". Without them, Lua would read `Hi` as the name of an instruction, and it knows none called that. The quotes themselves are not shown.

**Where it goes.** Printed text goes to the program's **output**: the lines a program shows while it runs. In Oxide, the **Run** button runs `src/main.lua` and shows its output in a console.

```lua
print("Bikes for rent")
print("Helmets included")
```

**One line each.** Every `print` starts a new line of output. Two prints give two lines, in the order they are written, because the program runs top to bottom.

**Who checks.** Saving runs the quest's checks. They run your program, collect what it printed and compare it with what the shop asked for, letter by letter, so a missing `!` counts.

## Your turn

- Make the first `print` show `Welcome to the bike shop!`.
- Make the second `print` show `Open 9 to 5`.
- Press Run to see the screen, then save and watch both objectives turn green.
