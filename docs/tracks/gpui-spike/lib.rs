use gpui::{Context, IntoElement, Render, Window, div, prelude::*, px, rgb};

pub struct StatsBar {
    pub clicks: u32,
}

impl Render for StatsBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_2()
            .h(px(32.))
            .child(
                div()
                    .id("pill-cpu")
                    .debug_selector(|| "pill-cpu".into())
                    .w(px(60.))
                    .bg(rgb(0x303030))
                    .child("CPU 12%")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.clicks += 1;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .debug_selector(|| "pill-ram".into())
                    .w(px(60.))
                    .child("RAM 40%"),
            )
    }
}
