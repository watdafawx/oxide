You have built the same row twice and read about the third route. None of them is the best GUI library; each one is the best at something. Here is how to choose, and the choice this track makes.

**egui for speed.** A tool, a debug panel, a settings screen, an overlay inside a game. One function, a widget per line, ready-made sliders and tables, and a test that clicks by label. You can have a working window in an evening.

**GPUI for products.** Something people keep open for hours: long lists, lots of text, driven from the keyboard. Views and `cx.notify()` take longer to learn, and you build your own buttons, but you get kept views, keyboard actions and focus built in, and the smallest memory of the three.

**Web for looks.** A designer, a team that knows CSS, a web UI that already exists, or a screen that must also open in a phone's browser. You pay in memory and a second language.

**What a task manager needs.** It stays open for hours, should run from the keyboard, and must stay light. egui with glow was close: it opened half a second faster and idled a few points lower. GPUI held the least memory and has keyboard actions and focus built in, made for apps people keep open, like Zed. That is the trade, and this track takes GPUI's side.

**egui is not gone.** Your model has no GUI in it, so the last world of this track draws the same task manager in egui. Then you measure your own two builds against each other.

**Why this was cheap.** Keep the words and the rules in plain Rust (`ProcessRow`, the sort, the search), and let the GUI only draw. The boss next proves it: one model, two faces.
