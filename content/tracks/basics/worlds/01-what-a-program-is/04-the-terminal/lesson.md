Oxide runs your code with one button. Most programmers run theirs by typing commands into a window called a terminal, and nearly every tutorial you will read assumes you know it.

**A window for commands.** A **terminal** (also called a console, shell or command line) is a window where you type a command, press Enter, and read the text that comes back. It is the oldest way to talk to a computer, and still the most direct.

**A command.** A command is the name of a program, followed by what it should work on. `lua main.lua` means "start the program called lua and give it the file main.lua". What comes back is that program's output.

```text
$ lua main.lua
Welcome to the bike shop!
Open 9 to 5
```

**The prompt.** The `$` at the start of a line is the **prompt**: the terminal's way of saying "ready, type here". You do not type it. Tutorials show it so you can tell the commands apart from what the computer printed.

**Folders and paths.** Files live in folders, and a **path** like `src/main.lua` is the route to a file: the folder `src`, then the file `main.lua` inside it. A command looks for files starting from the folder the terminal is currently in.

**The same in other languages.** For Rust you would type `cargo run` (compile, then run) or `cargo test` (run the checks); for Java, `gradlew test`. Oxide types these for you behind its buttons, so you can learn the ideas first and the commands when you need them.

## Your turn

Answer the quiz, then start the next quest.
