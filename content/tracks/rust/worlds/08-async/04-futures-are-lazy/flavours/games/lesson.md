A bug report: "sodium.jar never installs, and there is no error". The launcher calls the install function, so why did nothing happen? The answer is how futures run, and it is the same rule that makes a thousand idle downloads cost almost nothing.

**Calling builds, awaiting runs.** `let pending = install_jar(3);` runs none of the function's body. It builds a future and stops. Only when something awaits it (or spawns it, quest 07) does its first line run. A future is **lazy**: it waits to be asked.

**What poll means.** The runtime drives a future by **polling** it: calling it with "go as far as you can". The future runs until it reaches an `.await` on something that isn't ready, answers "not ready", and returns. The runtime moves on to another task.

**Being woken.** With each poll the future also gets a **waker**, a handle that means "poll me again". When the answer arrives, the network code calls the waker and the runtime polls that future once more. It carries on right after the `.await` where it paused. Nobody checks in a loop, so waiting costs nothing.

```rust
async fn sync_modpack() {
    install_jar(1);       // warning: unused implementer of `Future` that must be used
    install_jar(2).await; // this one runs
}
```

**A forgotten await is silent.** The future from `install_jar(1)` is dropped at the `;` and its body never runs. There is no error and no panic, only the warning above. When something async "never happens", look for a missing `.await` first.

**For the curious: a state machine.** The compiler turns each async fn into a struct that holds its local variables plus a number saying which `.await` it is paused at. That is why a task is small: it is exactly as big as what it must remember. Being lazy also makes cancelling cheap, since dropping that struct stops the work (quest 11).

## Your turn

- Answer the two questions. No code this time.
