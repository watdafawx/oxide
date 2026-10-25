The shop added a third line with today's price. Now the screen shows two lines and then stops with a red message. Programmers see messages like this all day; the skill is reading them, not fearing them.

**The message.** Save, and the objectives turn red with this message. Read it left to right: it is a sentence about your code, not noise.

```text
src/main.lua:4: attempt to call global 'prnt' (a nil value)
```

**Where.** The part before the first colon is the file, `src/main.lua`. The number after it is the line, `4`. Always look at that line first: the mistake is on it, or just above it.

**What.** The rest says what went wrong. To **call** something means to run it by its name, the way `print(...)` runs print. `'prnt'` is the name Lua tried, and **nil** is Lua's word for "nothing": no instruction has that name.

**Why only now.** Lines 2 and 3 ran fine and printed their text. The program stopped at the first line it could not carry out. That is the interpreter from two quests ago: it finds a mistake only when it reaches it, so this is a runtime error.

**Read, then fix.** The habit for every error: read the first message, go to the line it names, fix one thing, save, read again. Changing several things at once makes it hard to tell which change helped.

## Your turn

- Save once and read the message. Tap **why?** next to a red objective to see the full text.
- Go to the line the message names and fix the spelling.
- Save. The program runs to the end and both objectives turn green.
