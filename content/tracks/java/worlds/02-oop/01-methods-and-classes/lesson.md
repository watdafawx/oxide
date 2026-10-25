This world builds things that remember: a counter, an account, a ledger. Before that, two words you have used since your first quest need a proper meaning, because everything here is made of them.

**A named list of steps.** `public static int doubled(int n) { return n * 2; }` is a **method**: a list of steps with a name. The parts you know are the name (`doubled`), what goes in (`int n`) and what comes out (`int`, handed back by `return`).

**Running it.** Writing the name with values in brackets, `doubled(21)`, is a **call**. Java jumps into the method, runs its steps with `n` set to 21, and the call is replaced by what came back: 42. The checker has been calling your methods this way all along.

**Why name steps at all.** A name lets you write the steps once and use them anywhere. When a rule changes, you fix it in one place, and every call gets the fix.

**The box around them.** Every file you wrote started with `public class Badge { ... }`. A **class** has been a box that holds methods, and its name matches the file. Until now that is all it did.

**What changes in this world.** A class can also be a plan for things that remember values, like a counter that is still at 2 after you pressed it twice. The next quest builds the first one.
