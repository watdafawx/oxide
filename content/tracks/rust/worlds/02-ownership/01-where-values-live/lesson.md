Every quest in the last world handed numbers around freely. This world is about text and lists, and they behave differently. The reason is where values live while a program runs, so here is the picture first.

**Memory is a long street.** While a program runs, its values sit in the computer's **memory**: a long row of numbered cells. A cell's number is its **address**, like a house number. (Programming 101 draws this street from zero: [Numbered boxes](/quest/basics/07-how-the-computer-holds-things/01-numbered-boxes).)

**The stack.** Each function gets a small, tidy area called the **stack** for its own values, like the `u32` counts from the last world. Their size is known before the program runs (a `u32` is always 4 bytes), and the area is wiped the moment the function finishes.

**The heap.** Text can be 3 letters or 3 million, and a list can grow while the program runs. Those live on the **heap**: a big area where a program asks for room while it runs (it **allocates**) and must hand the room back afterwards (it **frees** it).

**A note with an address.** A `String` variable sits on the stack but holds only three numbers: the address of its letters on the heap, how many letters there are, and how much room is reserved. A value that says where another value is, like that address, is called a **pointer**.

```text
 stack (your function)              heap
 ┌──────────────────────────┐
 │ note: address ───────────┼──▶  m i l k ,   e g g s
 │       length 10, room 16 │
 │ count: 12                │
 └──────────────────────────┘
```

**Freeing is the hard part.** Forget to free and the program grows until the computer runs out: a **leak**. Free twice, or keep using memory after it was freed, and the program reads garbage or crashes. Bugs like these have caused decades of security holes in C and C++ programs.

**Two classic answers.** Java, Python, Lua and JavaScript use a **garbage collector**: a helper that runs now and then, finds heap memory nobody points at any more, and frees it. It is safe, but it costs pauses and extra memory. C makes you call `free` yourself: fast, and easy to get wrong.

**Rust's answer.** Rust gives every heap value exactly one **owner**, a variable, and frees the value the moment its owner goes away. The compiler checks the rules before the program runs, so there is no collector and no forgotten `free`. The rest of this world is those rules, one at a time.

## Your turn

- Answer the three questions. No code this time.
