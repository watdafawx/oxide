This world builds a server console: chat lines, player names, the list of who is online. In the crafting calculator you handed numbers around freely; text and lists behave differently. The reason is where values live while a program runs, so here is the picture first.

**Memory is a long street.** While a program runs, its values sit in the computer's **memory**: a long row of numbered cells. A cell's number is its **address**, like a house number. (Programming 101 draws this street from zero: [Numbered boxes](/quest/basics/07-how-the-computer-holds-things/01-numbered-boxes).)

**The stack.** Each function gets a small, tidy area called the **stack** for its own values, like a player's `u32` level. Their size is known before the program runs (a `u32` is always 4 bytes), and the area is wiped the moment the function finishes.

**The heap.** A chat line can be "hi" or a 200-letter rant, and the player list grows every time someone joins. Those live on the **heap**: a big area where a program asks for room while it runs (it **allocates**) and must hand the room back afterwards (it **frees** it).

**A note with an address.** A `String` variable sits on the stack but holds only three numbers: the address of its letters on the heap, how many letters there are, and how much room is reserved. A value that says where another value is, like that address, is called a **pointer**.

```text
 stack (your function)              heap
 ┌──────────────────────────┐
 │ line: address ───────────┼──▶  < S t e v e >   h i
 │       length 10, room 16 │
 │ level: 12                │
 └──────────────────────────┘
```

**Freeing is the hard part.** Forget to free and the server grows until the machine runs out: a **leak**. Free twice, or keep using memory after it was freed, and the program reads garbage or crashes. Bugs like these have caused decades of security holes in C and C++ servers.

**Two classic answers.** Minecraft itself is written in Java, which uses a **garbage collector**: a helper that runs now and then, finds heap memory nobody points at any more, and frees it. It is safe, but it costs pauses (some of those lag spikes) and extra memory. C makes you call `free` yourself: fast, and easy to get wrong.

**Rust's answer.** Rust gives every heap value exactly one **owner**, a variable, and frees the value the moment its owner goes away. The compiler checks the rules before the program runs, so there is no collector and no forgotten `free`. The rest of this world is those rules, one at a time.

## Your turn

- Answer the three questions. No code this time.
