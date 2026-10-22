use gpui::{Modifiers, TestAppContext, px, size};
use kinetik_spike::StatsBar;
use std::time::Duration;

#[gpui::test]
fn pills_are_drawn_side_by_side(cx: &mut TestAppContext) {
    let (_bar, cx) = cx.add_window_view(|_, _| StatsBar { clicks: 0 });
    cx.run_until_parked();
    let cpu = cx.debug_bounds("pill-cpu").expect("cpu pill drawn");
    let ram = cx.debug_bounds("pill-ram").expect("ram pill drawn");
    assert_eq!(cpu.size.width, px(60.));
    assert!(ram.origin.x > cpu.origin.x, "ram right of cpu: {cpu:?} {ram:?}");
    assert_eq!(cpu.origin.y, ram.origin.y);
}

#[gpui::test]
fn clicking_cpu_pill_counts(cx: &mut TestAppContext) {
    let (bar, cx) = cx.add_window_view(|_, _| StatsBar { clicks: 0 });
    cx.run_until_parked();
    let cpu = cx.debug_bounds("pill-cpu").unwrap();
    cx.simulate_click(cpu.center(), Modifiers::none());
    assert_eq!(bar.read_with(cx, |b, _| b.clicks), 1);
}

#[gpui::test]
fn resize_and_timer_work(cx: &mut TestAppContext) {
    let (_bar, vcx) = cx.add_window_view(|_, _| StatsBar { clicks: 0 });
    vcx.simulate_resize(size(px(200.), px(40.)));
    vcx.run_until_parked();
    assert!(vcx.debug_bounds("pill-cpu").is_some());
    let exec = cx.executor();
    let t = exec.timer(Duration::from_secs(1));
    exec.advance_clock(Duration::from_secs(1));
    exec.block_test(t);
}
