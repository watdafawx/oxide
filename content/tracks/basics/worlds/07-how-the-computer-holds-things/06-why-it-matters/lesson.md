This world's ideas are exactly where Rust and Java go their separate ways. Here is what to look for when you get there, so their strict rules come with a reason attached.

**Who cleans up.** Memory that is never freed is a **leak**: the program keeps growing until the computer runs out. Memory freed too early is worse: code then reads a spot that now holds something else. Lua and Java avoid both with a garbage collector, at the cost of a pause now and then.

**Rust: one owner.** Rust has no garbage collector. Instead, every value has exactly one **owner**, a variable, and the value is freed the moment its owner goes out of scope. Handing a list to another variable **moves** it there, and the old name can no longer be used. Two names for one list, like your undo bug, are allowed only in strict, checked forms called **borrowing**.

**Java: references everywhere.** In Java, every object is reached through a reference, exactly like Lua tables. Two variables can share one object, and `==` asks "the same object?" while `.equals(...)` compares contents: the split you met in "Same or alike".

**Nothing at all.** A reference that points nowhere is Lua's `nil` and Java's `null`. Using one by mistake is so common that the man who invented null later called it his "billion-dollar mistake". Rust has no null: a value that might be missing has the type `Option`, and the compiler makes you handle the "nothing" case.

**Copy or share.** Small values, like numbers, are copied; big ones, like lists, are shared or moved. Every language makes this choice somewhere. Once you know which choice it made, most surprising bugs stop being surprising.

## Your turn

Answer the quiz, then start the next quest.
