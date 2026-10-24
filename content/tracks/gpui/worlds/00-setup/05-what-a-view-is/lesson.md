The window from the last quest is empty, but it is not holding nothing: `main.rs` put an `Empty` in it with `cx.new(|_| Empty)`. Before the bar gets its first pill, here is what that is and what GPUI asks of it.

**A view lasts.** What fills a window is a **view**: a struct that GPUI keeps for as long as the window is open. `cx.new(|_| Empty)` hands GPUI the struct, and from then on GPUI holds it for you. The Live world shows why GPUI, not you, holds it.

**The Render trait.** A struct becomes a view by implementing `Render`, a trait with one method, `render`. GPUI calls it whenever it needs to draw the window and asks: what do you look like right now?

**What render gets.** `render(&mut self, window: &mut Window, cx: &mut Context<Self>)`. `self` is the view and its fields. `window` is the window it is drawn in, with its size and its light or dark look. `cx` is a context again, but not the `App` from `main.rs`: a `Context<Self>` is the app while you work on this one view, so later it can ask for this view to be redrawn.

**What it gives back.** `render` returns an **element**: a description of one frame, such as "a box holding the text CPU 12%". GPUI lays it out, paints it and throws it away. The next time it draws, it calls `render` again. The view lasts; the element is built fresh each time.

**Some kind of element.** The return type `impl IntoElement` is the `impl Trait` from the Rust track: "some type that can become an element; the compiler knows which". `Empty` gives back `div()`, an empty box, which is why the window is blank.

**For the curious.** GPUI does not compare the new description with the old one, the way a web framework's "virtual DOM" does. It rebuilds the elements of a window that needs drawing and paints that window again. That stays cheap because elements are small structs and the graphics card does the painting.
