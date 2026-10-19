**Why now:** soon your bot will count players, and some checks will go red because you counted 0 instead of 20. Red is not failure, it is the server telling you what is off. Learn to read it in ten seconds.

A failing assertion looks like this:

```
org.opentest4j.AssertionFailedError:
expected: 20
 but was: 0
    at PlayerCountTest.countsOnlinePlayers(PlayerCountTest.java:12)
```

- **expected** is what the test wants. **but was** is what *your* code returned. Here the bot returned 0 players when 20 were online.
- The last line says which test and **file:line** failed. Open it to see exactly what was checked.
- `UnsupportedOperationException: TODO` means the starter method is still a placeholder. Oxide shows it as "not written yet".
- A *compile error* is different: the code never ran. You will meet one in two quests.

Read the red from the top. Fix one thing, save, read again.

## Your turn

Answer the three questions below.
