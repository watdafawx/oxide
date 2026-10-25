The bar now has `.gap_2()` and `.h(px(32.))`, and soon `.px_2()` joins them. Two of those say `px` and one says `2`, and they mean three different things. Sorting them out now saves a confusing bug later.

**A scale of steps.** `gap_2` does not mean 2 pixels. Its number is a step on GPUI's **spacing scale**, where each step is a quarter of the base text size, the **rem**. The base is 16 pixels unless you change it, so `gap_1` is 4 pixels, `gap_2` is 8 and `gap_4` is 16.

**Why steps.** With only a few steps to pick from, every gap and margin in the app comes out from the same short list, so the bar, the widget and the settings window look like one app. And when the base text size grows, every step grows with it.

**Padding is room inside.** `.p_2()` puts step-2 room inside a box on all four sides, between its edge and its children. `.px_2()` does it left and right only, and `.py_2()` top and bottom only. Here the `x` and `y` are the two directions, as on a graph.

**`px(..)` is pixels.** The function `px(32.)` is something else: an exact length of 32 pixels, for when a size must not move, like the taskbar's 32-pixel height. It is a value you pass to a call such as `.h(..)`, not a call on the box.

**The trap.** `.px_2()` and `px(2.)` look almost the same. The first is padding on a box, about 8 pixels per side; the second is a 2-pixel length. If a box comes out with odd room inside it, check which one you wrote.

**For the curious.** The names come from Tailwind CSS, which uses the same steps: `p`, `px`, `py`, `m` for margin, `gap`, and `w_`/`h_` for sizes on the scale. `rems(1.)` gives a length in rems, the way `px(..)` gives one in pixels.
