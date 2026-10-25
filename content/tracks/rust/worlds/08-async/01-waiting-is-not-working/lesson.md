This world builds a podcast downloader. It asks a server for this week's 40 episodes, and for each one the program sends a request, then waits about half a second for the answer. Saving the bytes takes a few milliseconds. One after another, that is 20 seconds of mostly waiting, so you want to wait for all 40 at the same time.

**Waiting is not working.** Some jobs keep the processor busy: hashing a file, resizing a photo. That is **CPU work**, and world 07 split it across threads so several cores could share it. A download is different. The program sends a request and then has nothing to do until the network answers. That is **waiting on I/O** (input and output: network, disk), and more cores do not make a server answer faster.

**A thread per wait is costly.** World 07's threads would work: one thread per episode, each waiting on its own. But every thread is a worker the operating system has to manage. Each gets its own stack (Rust reserves 2 MB per thread by default), and the OS pays a switch every time it swaps one for another. 40 threads are fine; a server holding 10,000 open connections is not.

**Keep a note instead.** Waiting doesn't need a worker. It needs a note that says "come back to this when the answer arrives". In Rust that note is a **future**: a value that stands for a result that isn't here yet. One thread can hold thousands of notes and pick up whichever one is ready.

**Tasks and the runtime.** A future the program is working through is a **task**: a light piece of work that often costs a few hundred bytes instead of megabytes. The **runtime** is the part of the program that keeps the list of tasks, runs a few threads, and resumes each task when its answer arrives. Rust's standard library ships no runtime, so a program picks one as a crate. This world uses **tokio**, the most common one.

**One cook, many pots.** Picture a cook with ten pots on the stove. A cook who stands in front of each pot until it boils makes one dish at a time. A cook who turns to whichever pot is bubbling makes all ten together. Threads hire one cook per pot; async is one cook who never stands and waits.

**Which one when.** Async is for programs that wait a lot: network clients, servers, downloaders. For heavy CPU work, threads and rayon from world 07 are still the tool, and quest 06 shows how the two meet.

## Your turn

- Answer the questions. No code this time.
