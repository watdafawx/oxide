Your name tag needs a rule for how many letters fit on it. The test knows the rule; your code has it wrong. Reading the failure is how you find the fix.

**Red is information.** A red objective is not a punishment. Tests and the compiler tell you exactly what is wrong, and they are on your side. Most of learning Rust is reading what they say.

**Save once, change nothing.** Save the file as it is. The objective turns red. That is the starting point of every quest: red first, green when you fix it.

**Read both sides.** Click the red objective and Oxide shows what the test printed. A failed `assert_eq!` always shows two numbers:

```text
assertion `left == right` failed
  left: 7         <- what your code returned
 right: 8         <- what the test wanted
```

**The fix.** The `right` side is the answer the test wants. Put that number in the function, save, and the objective turns green.

**Compiler errors.** If the code does not build at all, the message appears above the objectives with the file and line. Read the first line first: it usually names the problem.

**If this is new.** Programming 101 explains tests and error messages from zero: [What a test is](/quest/basics/01-what-a-program-is/06-what-a-test-is) and [Read the red](/quest/basics/01-what-a-program-is/05-read-the-red).

## Your turn

- Save `max_tag_len` once without changing it. Click the red objective and find `left` and `right`.
- Change the number in `max_tag_len` to match `right`. Save.
