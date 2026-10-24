Kinetik's web dashboard is off by default, and even when it is on, it is for this PC only unless you allow other devices. Which devices can connect is decided by the address the server listens on.

**Addresses and ports.** An **IP address** names a machine on a network. A **port**, a number from 0 to 65535, names one program on that machine, so many servers can share one PC. `127.0.0.1:8787` is port 8787 on this PC, Kinetik's default.

**Loopback only.** `127.0.0.1`, `Ipv4Addr::LOCALHOST` in Rust, always means "this PC". A server listening there cannot be reached from any other machine, whatever the firewall says. That is the safe default.

**Every network card.** `0.0.0.0`, `Ipv4Addr::UNSPECIFIED`, means "every address this PC has", so your phone on the same Wi-Fi can open the page. Kinetik uses it only with "share on network" switched on, and still behind the key.

**Address plus port.** `SocketAddr::from((ip, port))` pairs an address with a port, from a tuple of the two:

```rust
let fixed = SocketAddr::from((Ipv4Addr::LOCALHOST, 8787)); // 127.0.0.1:8787
```

**Pick with an if.** In Rust an `if` gives back a value, so `let ip = if .. { .. } else { .. };` chooses the address in one line.

**The firewall too.** The first time a program listens on `0.0.0.0`, Windows asks whether to allow it. On `127.0.0.1` it never asks: nothing outside can reach it anyway.

## Your turn

- In `bind_addr`, pick `Ipv4Addr::LOCALHOST`, or `Ipv4Addr::UNSPECIFIED` when `share_on_lan` is on.
- Give back that address paired with `port`. Save.
