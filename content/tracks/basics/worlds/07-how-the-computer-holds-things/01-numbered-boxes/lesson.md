A drawing app holds a picture: thousands of colours that must live somewhere while the program runs. Knowing where values live explains the strangest bug of the next quest, and most of what makes Rust different.

**Memory.** While a program runs, its values live in the computer's **memory** (RAM): a huge row of tiny storage cells, each holding a small number. Every cell has a number of its own, its **address**, like the houses on a very long street.

**A name is a label.** When you write `local width = 16`, Lua puts 16 somewhere in memory and remembers that `width` means "that spot". The name is for you; the computer itself only deals in addresses.

**Big things live elsewhere.** A number is small and sits right where its name is. A table can grow to millions of items, so Lua keeps it in a separate area and the name holds only its address: a note saying "the table is over there". That note is called a **reference** (some languages call it a **pointer**).

**Gone when unused.** When nothing refers to a table any more, its memory can be reused. Lua and Java notice that and clean up on their own; the part that does it is called the **garbage collector**. Rust has none: it frees each value at a moment it works out while compiling.

**Stack and heap.** Languages like Rust split memory in two. The **stack** holds a function's small, fixed-size values and is cleared the moment the function returns. The **heap** holds things that grow or must outlive the function, like lists and text, and someone has to free them.

## Your turn

Answer the quiz, then start the next quest.
