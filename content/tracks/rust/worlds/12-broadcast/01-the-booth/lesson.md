This world builds `hud-server`: the Rust program that sits between Rocket League and everything the viewers see on an esports broadcast. Before any code, here is the room it runs in and the words the people in it use. Every later quest builds one box of this picture.

**The game talks.** Rocket League's official **Stats API** is a TCP port (49123) that the game opens on the broadcast PC. Through it the game sends every event of the match as JSON: the clock and score several times a second, a goal, a demolition. Older setups use **BakkesMod SOS**, a community plugin that sends the same events with other names.

**The HUD shows.** The **HUD** (heads-up display) is the graphics drawn over the game for viewers. Its main piece is the **scorebug**: the small bar with the team names, the score and the clock. A **stinger** is a short full-screen animation played on a moment, like a goal or a hat-trick.

**OBS paints it.** OBS is the program that mixes the game, the cameras and the graphics into the stream. Each HUD graphic is a web page inside OBS, called a **browser source**. hud-server pushes the latest state to those pages over a WebSocket, so the scorebug changes the moment the score does.

**Cues go out.** A **cue** means "this just happened, do your part". Notch (live broadcast graphics) and Resolume (video on the stage screens) take cues as **OSC**, Open Sound Control: tiny UDP packets with an address like `/notch/goal`. **Companion** is a program that presses buttons on a **Stream Deck**, the keypad of small screens the crew runs the show with; hud-server presses them over HTTP.

**The producer.** The **producer** decides what goes on air. They edit `triggers.json`, the file that says which cue sends what, and they never touch the Rust code. That is why the config has to be readable and its errors have to say where the mistake is.

**The whole path.** One line in, four lines out:

```text
Rocket League --TCP :49123--> hud-server --WebSocket--> OBS browser sources (scorebug, stingers)
                                          --OSC / UDP--> Notch, Resolume
                                          --HTTP-------> Companion --> Stream Deck buttons
                                          --Tauri------> the crew's control window
```

**Never hiccup on air.** One broken frame from the game, one slow overlay page or one big photo job must never stop the scorebug. That single rule explains most of the choices ahead: buffers for half-read data, channels between parts, time passed in as a value, heavy work moved off the async runtime.

**Made-up data.** Every quest runs on a recorded, invented match: Blue Herons (Wren, Fen, Ash) against Orange Owls (Pike, Moss, Teal). No real players, no real events.
