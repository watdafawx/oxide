Kinetik's web dashboard is "off by default", and even on, it is for this PC unless you allow other devices. Which devices can connect is decided by the address the server listens on.

**Loopback only.** `127.0.0.1` is "this PC". A server bound there cannot be reached from any other machine, whatever the firewall says. That is the safe default.

**Every network card.** `0.0.0.0` means "every address this PC has": your phone on the same Wi-Fi can open it. Only with "share on network" switched on, and still behind the key.

**Address plus port.** `SocketAddr::from((ip, port))` pairs them:

```rust
let ip = if share_on_lan { Ipv4Addr::UNSPECIFIED } else { Ipv4Addr::LOCALHOST };
```

**The firewall too.** The first time a program listens on 0.0.0.0, Windows asks whether to allow it. On 127.0.0.1 it never asks: nothing outside can reach it.

## Your turn

- In `bind_addr`, pick `Ipv4Addr::LOCALHOST` or `Ipv4Addr::UNSPECIFIED` by `share_on_lan`.
- Give back the `SocketAddr` with `port`. Save.
