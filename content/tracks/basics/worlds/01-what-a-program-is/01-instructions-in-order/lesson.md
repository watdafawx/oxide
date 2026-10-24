A bike shop wants a screen by the door that says hello and shows the opening hours. In this world you write it. Before the first line, it helps to know what a program actually is, because every later quest builds on this picture.

**A list of steps.** A program is a list of instructions written down as text, like a recipe. The computer does exactly what the text says and nothing more. It cannot guess what you meant, so the text has to say it.

**Top to bottom.** The computer runs the instructions one at a time, from the first line to the last. Line 2 starts only after line 1 is done, so order matters: put the hours above the greeting and the screen shows the hours first. (Text after `--` is a note for people, which Lua skips.)

```lua
print("Welcome to the bike shop!")   -- runs first: shows this line of text
print("Open 9 to 5")                  -- runs second
```

**Source code.** The text you write is called **source code**, or just code. It is a plain text file; Lua files end in `.lua`. On its own the file does nothing, the same way a recipe on paper cooks nothing.

**Running.** **Running** a program (also called executing it) means handing that text to the computer to carry out. Every run starts fresh from the top. Nothing is remembered from the last run unless the program saved it somewhere.

**Why so strict.** Computers are fast and never get bored, but they have no common sense. That is why code is fussy about spelling and punctuation: a word that is almost right is, to the computer, a different word it has never heard of.

**This track.** Every idea here is shown in **Lua**, a small language that games like Factorio and Roblox use for mods and scripts. Values, decisions, loops, functions and lists work the same way in Rust, Java and almost every other language; only the spelling changes, and the lessons point out where.

## Your turn

Answer the quiz, then start the next quest.
