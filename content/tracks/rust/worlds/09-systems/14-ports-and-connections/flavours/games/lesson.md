Back to the pinger. Everything so far worked on bytes already in memory. Now they have to travel between your launcher and a Minecraft server, usually on two different computers, and that needs a few words before the first line of network code.

**Client and server.** A **server** is a program that starts first and waits to be asked something: the Minecraft server. A **client** is a program that reaches out and asks: the launcher, or your pinger.

**An address.** Every computer on a network has an **IP address**, four numbers like `192.168.50.3` (the four bytes from the shifting quest). One address is special: `127.0.0.1`, also called **localhost**, always means "this same computer", which is how you join a server running on your own PC. The tests use it, so no byte ever leaves your machine.

**A port.** One computer can run many servers at once, so each one listens on a numbered **port**, from 0 to 65535: a `u16`. Minecraft's default is 25565, so a full address looks like `192.168.50.3:25565`. When a program asks for port `0`, the operating system picks any free one, which tests use so they never collide.

**A connection.** **TCP** is how Minecraft Java talks. The client **connects** to an address and port, the server **accepts**, and now both sides hold one end of a **connection**. Bytes written at one end come out at the other, all of them and in order, or you get an error.

**A stream, not messages.** TCP delivers a stream of bytes, not separate messages. Two writes of `hel` and `lo` may arrive as one read of `hello`, or as `h` and `ello`. That is why every Minecraft packet starts with its length: the reader needs it to know where one packet ends.

**UDP.** The other way is **UDP**: no connection, just single packets called **datagrams**, each sent to an address. It is faster to start, but a datagram can be lost and nobody tells you. Bedrock Edition plays over it, and Java servers can answer an optional UDP status query.

```text
client (your pinger)                   Minecraft server
192.168.50.20, any free port   --->   192.168.50.3, port 25565
          connect           ------------>   accept
          write handshake   ------------>   read
          read              <------------   write the status
```

**The word for an end.** One end of a connection, or a UDP endpoint, is a **socket**. In Rust they are `TcpListener` (a server waiting), `TcpStream` (one connection) and `UdpSocket`: the safe wrappers around the operating system's C functions from the aside.

## Your turn

Answer the questions, then start the next quest.
