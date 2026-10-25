Back to the checker. Everything so far worked on bytes already in memory. Now they have to travel between two programs, usually on two different computers, and that needs a few words before the first line of network code.

**Client and server.** A **server** is a program that starts first and waits to be asked something. A **client** is a program that reaches out and asks. Your checker is a client; whatever it checks is a server.

**An address.** Every computer on a network has an **IP address**, four numbers like `192.168.50.3` (the four bytes from the shifting quest). One address is special: `127.0.0.1`, also called **localhost**, always means "this same computer". The tests use it, so no byte ever leaves your machine.

**A port.** One computer can run many servers at once, so each one listens on a numbered **port**, from 0 to 65535: a `u16`. Web servers usually use 443, so a full address looks like `192.168.50.3:443`. When a program asks for port `0`, the operating system picks any free one, which tests use so they never collide.

**A connection.** **TCP** is the way most programs talk. The client **connects** to an address and port, the server **accepts**, and now both sides hold one end of a **connection**. Bytes written at one end come out at the other, all of them and in order, or you get an error.

**A stream, not messages.** TCP delivers a stream of bytes, not separate messages. Two writes of `hel` and `lo` may arrive as one read of `hello`, or as `h` and `ello`. That is why protocols put a length in front of every message: the reader needs it to know where one ends.

**UDP.** The other way is **UDP**: no connection, just single packets called **datagrams**, each sent to an address. It is faster to start, but a datagram can be lost and nobody tells you. Games use it for positions, where the next update replaces a lost one anyway.

```text
client (your checker)                  server
192.168.50.20, any free port   --->   192.168.50.3, port 443
          connect           ------------>   accept
          write "status?"   ------------>   read
          read              <------------   write the answer
```

**The word for an end.** One end of a connection, or a UDP endpoint, is a **socket**. In Rust they are `TcpListener` (a server waiting), `TcpStream` (one connection) and `UdpSocket`: the safe wrappers around the operating system's C functions from the aside.

## Your turn

Answer the questions, then start the next quest.
