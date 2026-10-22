// Spike R3 + R5: a transparent PopUp bar docked left of the tray, topmost, plus
// tray-icon and global-hotkey events inside GPUI's message loop. Prints a report and quits.
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};
use gpui::{
    App, Application, Bounds, Context, Render, Window, WindowBackgroundAppearance, WindowBounds,
    WindowKind, WindowOptions, div, point, prelude::*, px, rgb, size,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::time::Duration;
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use windows::Win32::Foundation::{HWND, LPARAM, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, SendInput,
    VIRTUAL_KEY, VK_CONTROL, VK_F9, VK_MENU, VK_SHIFT,
};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::w;

struct Probe;

impl Render for Probe {
    fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Left half: magenta pill. Right half: nothing (should show the taskbar through).
        div()
            .size_full()
            .flex()
            .flex_row()
            .child(div().w(px(100.)).h_full().bg(rgb(0xff00ff)))
    }
}

struct Backdrop(u32);
impl Render for Backdrop {
    fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(self.0))
    }
}

fn rect_of(h: HWND) -> RECT {
    let mut r = RECT::default();
    unsafe { GetWindowRect(h, &mut r).unwrap() };
    r
}

fn pixel(x: i32, y: i32) -> u32 {
    use windows::Win32::Graphics::Gdi::*;
    unsafe {
        let screen = GetDC(None);
        let mem = CreateCompatibleDC(Some(screen));
        let bmp = CreateCompatibleBitmap(screen, 1, 1);
        let old = SelectObject(mem, bmp.into());
        BitBlt(mem, 0, 0, 1, 1, Some(screen), x, y, SRCCOPY | CAPTUREBLT).unwrap();
        let c = GetPixel(mem, 0, 0).0;
        SelectObject(mem, old);
        let _ = DeleteObject(bmp.into());
        let _ = DeleteDC(mem);
        ReleaseDC(None, screen);
        c
    }
}

fn key(vk: VIRTUAL_KEY, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                dwFlags: if up { KEYEVENTF_KEYUP } else { KEYBD_EVENT_FLAGS(0) },
                ..Default::default()
            },
        },
    }
}

fn own_window_of_class(class: windows::core::PCWSTR) -> Option<HWND> {
    let me = std::process::id();
    let mut prev: Option<HWND> = None;
    loop {
        let h = unsafe { FindWindowExW(None, prev, class, None) }.ok()?;
        let mut pid = 0u32;
        unsafe { GetWindowThreadProcessId(h, Some(&mut pid)) };
        if pid == me {
            return Some(h);
        }
        prev = Some(h);
    }
}

fn main() {
    let t0 = std::time::Instant::now();
    Application::new().run(move |cx: &mut App| {
        // --- taskbar geometry (physical pixels)
        let taskbar = unsafe { FindWindowW(w!("Shell_TrayWnd"), None) }.expect("taskbar");
        let tray = unsafe { FindWindowExW(Some(taskbar), None, w!("TrayNotifyWnd"), None) }
            .expect("tray");
        let tb = rect_of(taskbar);
        let tr = rect_of(tray);
        let dpi = unsafe { GetDpiForWindow(taskbar) } as f32;
        let scale = dpi / 96.0;
        let bar_w_phys = (200.0 * scale) as i32;
        let left_phys = tr.left - bar_w_phys - (8.0 * scale) as i32;
        let top_phys = 1440 - 48 - 60; // on-screen spot: this PC's taskbar auto-hides
        let h_phys = 48;
        println!("R3 taskbar={:?} tray={:?} dpi={dpi} scale={scale}", (tb.left, tb.top, tb.right, tb.bottom), (tr.left, tr.top, tr.right, tr.bottom));

        // pixel in the right (transparent) half, before the window exists
        let probe_x = left_phys + (150.0 * scale) as i32;
        let probe_y = top_phys + h_phys / 2;
        let before = pixel(probe_x, probe_y);

        let bounds = Bounds::new(
            point(px(left_phys as f32 / scale), px(top_phys as f32 / scale)),
            size(px(200.), px(h_phys as f32 / scale)),
        );
        let backdrop = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    kind: WindowKind::PopUp,
                    focus: false,
                    ..Default::default()
                },
                |_, cx| cx.new(|_| Backdrop(0xffffff)),
            )
            .unwrap();
        let handle = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    kind: WindowKind::PopUp,
                    is_movable: false,
                    is_resizable: false,
                    focus: false,
                    window_background: WindowBackgroundAppearance::Transparent,
                    ..Default::default()
                },
                |_, cx| cx.new(|_| Probe),
            )
            .unwrap();
        for d in cx.displays() { println!("R3 display id={:?} bounds={:?}", d.id(), d.bounds()); }
        println!("R3 primary={:?}", cx.primary_display().map(|d| d.bounds()));
        println!("R3 window opened after {:?}", t0.elapsed());

        // --- R5: tray icon + hotkey created on GPUI's main thread
        let rgba: Vec<u8> = (0..16 * 16).flat_map(|_| [0x20, 0xc0, 0xff, 0xff]).collect();
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("Kinetik spike")
            .with_icon(Icon::from_rgba(rgba, 16, 16).unwrap())
            .build();
        println!("R5 tray icon created: {}", tray_icon.is_ok());
        let hotkeys = GlobalHotKeyManager::new().unwrap();
        let hk = HotKey::new(
            Some(Modifiers::CONTROL | Modifiers::ALT | Modifiers::SHIFT),
            Code::F9,
        );
        println!("R5 hotkey registered: {:?}", hotkeys.register(hk).map(|_| ()));

        cx.spawn(async move |cx| {
            let _keep = (tray_icon, hotkeys);
            cx.background_executor().timer(Duration::from_millis(1500)).await;

            let hwnd = handle
                .update(cx, |_, window, _| match window.window_handle().unwrap().as_raw() {
                    RawWindowHandle::Win32(h) => HWND(h.hwnd.get() as *mut _),
                    _ => unreachable!(),
                })
                .unwrap();
            let r = rect_of(hwnd);
            let ex = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) } as u32;
            println!(
                "R3 hwnd rect={:?} wanted={:?} ex_style={ex:#x} topmost_before={}",
                (r.left, r.top, r.right, r.bottom),
                (left_phys, top_phys, left_phys + bar_w_phys, top_phys + h_phys),
                ex & WS_EX_TOPMOST.0 != 0
            );
            unsafe {
                SetWindowPos(hwnd, Some(HWND_TOPMOST), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE).unwrap()
            };
            let r = rect_of(hwnd);
            println!("R3 after SetWindowPos rect={:?}", (r.left, r.top, r.right, r.bottom));
            let n = handle.update(cx, |_, window, _| window.viewport_size()).unwrap();
            println!("R3 gpui viewport after move={:?}", n);
            let ex = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) } as u32;
            println!("R3 topmost_after={}", ex & WS_EX_TOPMOST.0 != 0);

            // z-order: is our window above the taskbar?
            let mut above = false;
            let mut h = unsafe { GetWindow(hwnd, GW_HWNDNEXT) }.ok();
            while let Some(x) = h {
                if x == taskbar {
                    above = true;
                    break;
                }
                h = unsafe { GetWindow(x, GW_HWNDNEXT) }.ok();
            }
            println!("R3 above_taskbar={above}");

            cx.background_executor().timer(Duration::from_millis(500)).await;
            let n = handle.update(cx, |_, window, _| window.viewport_size()).unwrap();
            println!("R3 gpui viewport 500ms later={:?}", n);
            let pill_x = left_phys + (50.0 * scale) as i32;
            let pill = pixel(pill_x, probe_y);
            let after = pixel(probe_x, probe_y);
            println!("R3 pill_pixel={pill:#08x} (magenta=0xff00ff as BGR)");
            let mut shown = vec![]; let mut hidden = vec![];
            for dx in [120, 150, 190] { shown.push(pixel(left_phys + dx, probe_y)); }
            unsafe { let _ = ShowWindow(hwnd, SW_HIDE); }
            cx.background_executor().timer(Duration::from_millis(400)).await;
            for dx in [120, 150, 190] { hidden.push(pixel(left_phys + dx, probe_y)); }
            unsafe { let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE); }
            for colour in [0xffffffu32, 0x000000, 0xff0000] {
                backdrop.update(cx, |b, _, cx| { b.0 = colour; cx.notify(); }).unwrap();
                cx.background_executor().timer(Duration::from_millis(400)).await;
                let s = pixel(left_phys + 150, probe_y);
                unsafe { let _ = ShowWindow(hwnd, SW_HIDE); }
                cx.background_executor().timer(Duration::from_millis(400)).await;
                let h = pixel(left_phys + 150, probe_y);
                unsafe { let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE); }
                cx.background_executor().timer(Duration::from_millis(200)).await;
                println!("R3 backdrop {colour:06x}: through bar={s:06x} bar hidden={h:06x}");
            }
            println!("R3 transparent area shown={shown:x?} hidden={hidden:x?}");
            println!("R3 transparent_pixel before={before:#08x} after={after:#08x} same={}", before == after);

            // --- R5: fire events through the real message loop
            let inputs = [
                key(VK_CONTROL, false), key(VK_MENU, false), key(VK_SHIFT, false),
                key(VK_F9, false), key(VK_F9, true),
                key(VK_SHIFT, true), key(VK_MENU, true), key(VK_CONTROL, true),
            ];
            let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
            println!("R5 SendInput sent {sent} keys");
            match own_window_of_class(w!("tray_icon_app")) {
                Some(th) => unsafe {
                    PostMessageW(Some(th), 6002, WPARAM(0), LPARAM(WM_LBUTTONUP as isize)).unwrap();
                    println!("R5 posted WM_LBUTTONUP to tray window");
                },
                None => println!("R5 tray window not found"),
            }

            let mut hot = 0;
            let mut tray_clicks = 0;
            for _ in 0..20 {
                cx.background_executor().timer(Duration::from_millis(100)).await;
                while let Ok(e) = GlobalHotKeyEvent::receiver().try_recv() {
                    if e.id == hk.id() { hot += 1; }
                }
                while let Ok(e) = TrayIconEvent::receiver().try_recv() {
                    if matches!(e, TrayIconEvent::Click { .. }) { tray_clicks += 1; }
                }
            }
            println!("R5 hotkey_events={hot} tray_click_events={tray_clicks}");
            let mut p = POINT::default();
            let _ = unsafe { windows::Win32::UI::WindowsAndMessaging::GetCursorPos(&mut p) };
            println!("DONE total {:?}", t0.elapsed());
            cx.update(|cx| cx.quit()).ok();
        })
        .detach();
    });
}
