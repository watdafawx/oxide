Your mod needs things that remember: an item stack that knows it holds 17 logs, a mob with its own health, a chest with a log of what went in. Before that, two words you have used since your first quest need a proper meaning, because all of it is made of them.

**A named list of steps.** `public static int stacks(int items) { return items / 64; }` is a **method**: a list of steps with a name. The parts you know are the name (`stacks`), what goes in (`int items`) and what comes out (`int`, handed back by `return`).

**Running it.** Writing the name with values in brackets, `stacks(130)`, is a **call**. Java jumps into the method, runs its steps with `items` set to 130, and the call is replaced by what came back: 2. The checker has been calling your methods this way all along.

**Why name steps at all.** A name lets you write the steps once and use them anywhere. When a rule changes, say stacks of 16 for ender pearls, you fix it in one place, and every call gets the fix.

**The box around them.** Every file you wrote started with `public class Report { ... }`. A **class** has been a box that holds methods, and its name matches the file. Until now that is all it did.

**What changes in this world.** A class can also be a plan for things that remember values, like an item stack that still holds 17 logs after you looked away. The next quest builds the first one.
