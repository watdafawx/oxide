//! A Model Context Protocol server inside Oxide, so an agent can read, drive and
//! screenshot this window without desktop control.
//!
//! On in debug builds, or with `--mcp` / `OXIDE_MCP=1`. Listens on 127.0.0.1 only.
//! Streamable HTTP with plain JSON responses: POST JSON-RPC to
//! `http://127.0.0.1:47811/mcp`. `scripts/mcp.py` is the shell helper.
//!
//! Tools run JavaScript in the webview and get the value back through the
//! `mcp_reply` command, so `invoke` reaches every Tauri command the UI can call.

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::{Value, json};
use tauri::{AppHandle, Manager, State};

pub const DEFAULT_PORT: u16 = 47811;
const PROTOCOL: &str = "2025-06-18";
const JS_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Clone, Serialize)]
pub struct Event {
    pub seq: u64,
    pub name: String,
    pub payload: Value,
    pub at_ms: u64,
}

/// Reply slots for in-flight JavaScript, plus the log of emitted events.
pub struct Mcp {
    pending: Mutex<HashMap<u64, Sender<Result<Value, String>>>>,
    next_id: AtomicU64,
    events: Mutex<VecDeque<Event>>,
    seq: AtomicU64,
    event_arrived: Condvar,
    started: Instant,
}

impl Default for Mcp {
    fn default() -> Self {
        Self {
            pending: Mutex::new(HashMap::new()),
            next_id: AtomicU64::new(1),
            events: Mutex::new(VecDeque::new()),
            seq: AtomicU64::new(0),
            event_arrived: Condvar::new(),
            started: Instant::now(),
        }
    }
}

impl Mcp {
    /// Remember an event the app emitted to the UI.
    pub fn record(&self, name: &str, payload: Value) {
        let seq = self.seq.fetch_add(1, Ordering::SeqCst) + 1;
        let mut events = self.events.lock().unwrap();
        events.push_back(Event { seq, name: name.to_string(), payload, at_ms: self.started.elapsed().as_millis() as u64 });
        while events.len() > 200 {
            events.pop_front();
        }
        self.event_arrived.notify_all();
    }

    fn events_after(&self, after: u64) -> Vec<Event> {
        self.events.lock().unwrap().iter().filter(|e| e.seq > after).cloned().collect()
    }

    fn wait_event(&self, name: &str, after: u64, timeout: Duration) -> Option<Event> {
        let deadline = Instant::now() + timeout;
        let mut events = self.events.lock().unwrap();
        loop {
            if let Some(e) = events.iter().find(|e| e.seq > after && e.name == name) {
                return Some(e.clone());
            }
            let now = Instant::now();
            if now >= deadline {
                return None;
            }
            let (guard, _) = self.event_arrived.wait_timeout(events, deadline - now).unwrap();
            events = guard;
        }
    }
}

/// The webview reports a finished `eval` here.
#[tauri::command]
pub fn mcp_reply(state: State<Mcp>, id: u64, ok: bool, value: Value) {
    if let Some(tx) = state.pending.lock().unwrap().remove(&id) {
        let _ = tx.send(if ok { Ok(value) } else { Err(value.as_str().unwrap_or("error").to_string()) });
    }
}

pub fn enabled() -> bool {
    cfg!(debug_assertions) || std::env::args().any(|a| a == "--mcp") || std::env::var_os("OXIDE_MCP").is_some()
}

pub fn port() -> u16 {
    std::env::var("OXIDE_MCP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(DEFAULT_PORT)
}

/// Starts the HTTP thread. Needs `app.manage(Mcp::default())` first.
pub fn start(app: AppHandle, port: u16) -> anyhow::Result<()> {
    let server = tiny_http::Server::http(("127.0.0.1", port)).map_err(|e| anyhow::anyhow!("MCP port {port}: {e}"))?;
    std::thread::Builder::new().name("mcp".into()).spawn(move || {
        for mut req in server.incoming_requests() {
            let mut body = String::new();
            let _ = req.as_reader().read_to_string(&mut body);
            let (status, out) = handle(&app, &body);
            let mut resp = tiny_http::Response::from_string(out).with_status_code(status);
            resp.add_header(tiny_http::Header::from_bytes("Content-Type", "application/json").expect("static header"));
            let _ = req.respond(resp);
        }
    })?;
    log::info!("MCP server on http://127.0.0.1:{port}/mcp");
    Ok(())
}

fn text(s: impl Into<String>) -> Value {
    json!({ "type": "text", "text": s.into() })
}

fn pretty(v: &Value) -> Value {
    text(serde_json::to_string_pretty(v).unwrap_or_default())
}

fn tools() -> Value {
    let obj = |props: Value, required: &[&str]| json!({ "type": "object", "properties": props, "required": required });
    json!([
        { "name": "page", "description": "Current route, visible text and interactive elements (index, tag, text, testid). Prefer this over screenshot.",
          "inputSchema": obj(json!({}), &[]) },
        { "name": "screenshot", "description": "PNG of the window's client area.",
          "inputSchema": obj(json!({}), &[]) },
        { "name": "navigate", "description": "Go to a route such as /map or /quest/00-tutorial/01-hello.",
          "inputSchema": obj(json!({ "path": { "type": "string" } }), &["path"]) },
        { "name": "click", "description": "Click an element by CSS selector, by visible text (buttons/links, case-insensitive), or by index from `page`.",
          "inputSchema": obj(json!({ "selector": { "type": "string" }, "text": { "type": "string" }, "index": { "type": "integer" } }), &[]) },
        { "name": "type", "description": "Set an input's value (fires input/change) by CSS selector.",
          "inputSchema": obj(json!({ "selector": { "type": "string" }, "text": { "type": "string" } }), &["selector", "text"]) },
        { "name": "invoke", "description": "Call a Tauri command (see docs/API.md) and return its result.",
          "inputSchema": obj(json!({ "command": { "type": "string" }, "args": { "type": "object" } }), &["command"]) },
        { "name": "eval", "description": "Run JavaScript in the webview: an expression, or a function body with `return` (await allowed). Returns the JSON value.",
          "inputSchema": obj(json!({ "js": { "type": "string" } }), &["js"]) },
        { "name": "events", "description": "Events the app emitted to the UI (check:started, check:result, profile:changed, focus:*), with seq numbers.",
          "inputSchema": obj(json!({ "after": { "type": "integer", "description": "only events with seq > after" } }), &[]) },
        { "name": "wait_event", "description": "Block until an event with this name arrives (seq > after), or time out.",
          "inputSchema": obj(json!({ "name": { "type": "string" }, "after": { "type": "integer" }, "timeout_s": { "type": "number" } }), &["name"]) },
    ])
}

fn handle(app: &AppHandle, body: &str) -> (u16, String) {
    let Ok(msg) = serde_json::from_str::<Value>(body) else {
        return (400, json!({ "jsonrpc": "2.0", "id": null, "error": { "code": -32700, "message": "parse error" } }).to_string());
    };
    let method = msg["method"].as_str().unwrap_or("");
    let Some(id) = msg.get("id").cloned() else {
        return (202, String::new()); // notification
    };
    let result = match method {
        "initialize" => Ok(json!({
            "protocolVersion": msg["params"]["protocolVersion"].as_str().unwrap_or(PROTOCOL),
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "oxide", "version": env!("CARGO_PKG_VERSION") },
            "instructions": "Drives the Oxide window. Use `page` to read the screen and `events`/`wait_event` to follow checks; `invoke` calls any command in docs/API.md."
        })),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({ "tools": tools() })),
        "tools/call" => {
            let name = msg["params"]["name"].as_str().unwrap_or("");
            let args = match &msg["params"]["arguments"] {
                Value::String(s) => serde_json::from_str(s).unwrap_or(Value::Null),
                v => v.clone(),
            };
            Ok(match call(app, name, &args) {
                Ok(content) => json!({ "content": content }),
                Err(e) => json!({ "content": [text(e)], "isError": true }),
            })
        }
        _ => Err(json!({ "code": -32601, "message": format!("method not found: {method}") })),
    };
    let out = match result {
        Ok(r) => json!({ "jsonrpc": "2.0", "id": id, "result": r }),
        Err(e) => json!({ "jsonrpc": "2.0", "id": id, "error": e }),
    };
    (200, out.to_string())
}

const PAGE_JS: &str = r#"
const els = [...document.querySelectorAll('button, a[href], input, textarea, select, [role=button], [data-testid]')]
  .filter(el => el.getClientRects().length > 0)
  .map((el, i) => ({
    i, tag: el.tagName.toLowerCase(),
    text: (el.innerText || el.getAttribute('aria-label') || el.placeholder || '').trim().replace(/\s+/g, ' ').slice(0, 80),
    testid: el.dataset.testid || undefined,
    href: el.getAttribute('href') || undefined,
    value: ('value' in el && el.tagName !== 'BUTTON') ? String(el.value).slice(0, 80) : undefined,
    disabled: el.disabled || undefined,
  }));
return { url: location.pathname + location.search, title: document.title, text: document.body.innerText, elements: els };
"#;

fn find_js(args: &Value) -> Result<String, String> {
    if let Some(sel) = args["selector"].as_str() {
        return Ok(format!("document.querySelector({})", json!(sel)));
    }
    if let Some(i) = args["index"].as_u64() {
        return Ok(format!(
            "[...document.querySelectorAll('button, a[href], input, textarea, select, [role=button], [data-testid]')].filter(el => el.getClientRects().length > 0)[{i}]"
        ));
    }
    if let Some(t) = args["text"].as_str() {
        let t = json!(t.to_lowercase());
        return Ok(format!(
            "(() => {{ const all = [...document.querySelectorAll('button, a[href], [role=button], summary, label')].filter(el => el.getClientRects().length > 0); \
             const norm = el => (el.innerText || '').trim().replace(/\\s+/g, ' ').toLowerCase(); \
             return all.find(el => norm(el) === {t}) || all.find(el => norm(el).includes({t})) || null; }})()"
        ));
    }
    Err("give selector, text or index".into())
}

fn call(app: &AppHandle, name: &str, args: &Value) -> Result<Vec<Value>, String> {
    let mcp = app.state::<Mcp>();
    match name {
        "page" => Ok(vec![pretty(&run_js(app, PAGE_JS)?)]),
        "screenshot" => {
            let (w, h, png) = screenshot(app)?;
            Ok(vec![json!({ "type": "image", "data": png, "mimeType": "image/png" }), text(format!("{w}x{h}"))])
        }
        "navigate" => {
            let path = args["path"].as_str().ok_or("missing 'path'")?;
            run_js(app, &format!("setTimeout(() => location.assign({}), 30); return {};", json!(path), json!(path)))?;
            std::thread::sleep(Duration::from_millis(700));
            Ok(vec![pretty(&run_js(app, PAGE_JS)?)])
        }
        "click" => {
            let find = find_js(args)?;
            let v = run_js(
                app,
                &format!(
                    "const el = {find}; if (!el) return null; el.scrollIntoView({{ block: 'center' }}); el.click(); \
                     return {{ tag: el.tagName.toLowerCase(), text: (el.innerText || '').trim().slice(0, 80) }};"
                ),
            )?;
            if v.is_null() {
                return Err("no such element".into());
            }
            std::thread::sleep(Duration::from_millis(300));
            Ok(vec![text(format!("clicked {v}"))])
        }
        "type" => {
            let sel = args["selector"].as_str().ok_or("missing 'selector'")?;
            let t = args["text"].as_str().ok_or("missing 'text'")?;
            let v = run_js(
                app,
                &format!(
                    "const el = document.querySelector({}); if (!el) return null; \
                     const proto = el.tagName === 'TEXTAREA' ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype; \
                     const set = Object.getOwnPropertyDescriptor(proto, 'value').set; \
                     if (el.type === 'checkbox') {{ el.click(); }} else {{ set.call(el, {}); }} \
                     el.dispatchEvent(new Event('input', {{ bubbles: true }})); el.dispatchEvent(new Event('change', {{ bubbles: true }})); \
                     return el.type === 'checkbox' ? el.checked : el.value;",
                    json!(sel),
                    json!(t)
                ),
            )?;
            if v.is_null() {
                return Err("no such element".into());
            }
            Ok(vec![text(format!("value now {v}"))])
        }
        "invoke" => {
            let cmd = args["command"].as_str().ok_or("missing 'command'")?;
            let a = if args["args"].is_object() { args["args"].clone() } else { json!({}) };
            let v = run_js(app, &format!("return await window.__TAURI_INTERNALS__.invoke({}, {});", json!(cmd), a))?;
            Ok(vec![pretty(&v)])
        }
        "eval" => {
            let js = args["js"].as_str().ok_or("missing 'js'")?;
            Ok(vec![pretty(&run_js(app, js)?)])
        }
        "events" => {
            let after = args["after"].as_u64().unwrap_or(0);
            let evs = mcp.events_after(after);
            Ok(vec![pretty(&json!({ "latest_seq": mcp.seq.load(Ordering::SeqCst), "events": evs }))])
        }
        "wait_event" => {
            let name = args["name"].as_str().ok_or("missing 'name'")?;
            let after = args["after"].as_u64().unwrap_or(0);
            let timeout = Duration::from_secs_f64(args["timeout_s"].as_f64().unwrap_or(15.0));
            match mcp.wait_event(name, after, timeout) {
                Some(e) => Ok(vec![pretty(&serde_json::to_value(e).unwrap_or_default())]),
                None => Err(format!("no `{name}` event within {}s", timeout.as_secs_f64())),
            }
        }
        _ => Err(format!("unknown tool `{name}`")),
    }
}

/// Run `js` in the main webview and wait for its value. `js` is a function body
/// (use `return`; `await` is fine) or a bare expression.
fn run_js(app: &AppHandle, js: &str) -> Result<Value, String> {
    let mcp = app.state::<Mcp>();
    let id = mcp.next_id.fetch_add(1, Ordering::SeqCst);
    let (tx, rx) = channel();
    mcp.pending.lock().unwrap().insert(id, tx);
    let body = if js.contains("return") { js.to_string() } else { format!("return ({js});") };
    let wrapped = format!(
        "(async () => {{ let r; try {{ const v = await (async () => {{ {body}\n }})(); r = {{ id: {id}, ok: true, value: v === undefined ? null : v }}; }} \
         catch (e) {{ r = {{ id: {id}, ok: false, value: String((e && e.stack) || e) }}; }} \
         window.__TAURI_INTERNALS__.invoke('mcp_reply', r); }})();"
    );
    let win = app.get_webview_window("main").ok_or("no main window")?;
    win.eval(wrapped.as_str()).map_err(|e| e.to_string())?;
    match rx.recv_timeout(JS_TIMEOUT) {
        Ok(r) => r,
        Err(_) => {
            mcp.pending.lock().unwrap().remove(&id);
            Err("the webview did not answer in 20s (unserialisable value, or page navigated)".into())
        }
    }
}

/// Client area as PNG base64, drawn by the window itself (works behind other windows).
#[cfg(windows)]
fn screenshot(app: &AppHandle) -> Result<(u32, u32, String), String> {
    use base64::Engine;
    use windows::Win32::Foundation::RECT;
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::Storage::Xps::{PRINT_WINDOW_FLAGS, PrintWindow};
    use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

    let win = app.get_webview_window("main").ok_or("no main window")?;
    let hwnd = win.hwnd().map_err(|e| e.to_string())?;
    // SAFETY: plain GDI calls on our own live window; every object made here is released.
    let (w, h, mut buf) = unsafe {
        let mut r = RECT::default();
        GetClientRect(hwnd, &mut r).map_err(|e| e.to_string())?;
        let (w, h) = (r.right - r.left, r.bottom - r.top);
        if w <= 0 || h <= 0 {
            return Err("the window is minimized".into());
        }
        let wdc = GetDC(Some(hwnd));
        let mdc = CreateCompatibleDC(Some(wdc));
        let bmp = CreateCompatibleBitmap(wdc, w, h);
        let old = SelectObject(mdc, bmp.into());
        let ok = PrintWindow(hwnd, mdc, PRINT_WINDOW_FLAGS(1 | 2)).as_bool();
        let mut info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w,
                biHeight: -h,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut buf = vec![0u8; (w * h * 4) as usize];
        let lines = GetDIBits(mdc, bmp, 0, h as u32, Some(buf.as_mut_ptr().cast()), &mut info, DIB_RGB_COLORS);
        SelectObject(mdc, old);
        let _ = DeleteObject(bmp.into());
        let _ = DeleteDC(mdc);
        ReleaseDC(Some(hwnd), wdc);
        if !ok || lines == 0 {
            return Err("could not capture the window".into());
        }
        (w as u32, h as u32, buf)
    };
    for p in buf.as_chunks_mut::<4>().0 {
        p.swap(0, 2);
        p[3] = 255;
    }
    let mut png = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut png, w, h);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut writer = enc.write_header().map_err(|e| e.to_string())?;
        writer.write_image_data(&buf).map_err(|e| e.to_string())?;
    }
    Ok((w, h, base64::engine::general_purpose::STANDARD.encode(png)))
}

#[cfg(not(windows))]
fn screenshot(_: &AppHandle) -> Result<(u32, u32, String), String> {
    Err("screenshots are only available on Windows".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_and_wait() {
        let m = Mcp::default();
        m.record("check:started", json!({ "quest_id": "a" }));
        assert_eq!(m.events_after(0).len(), 1);
        assert!(m.wait_event("check:result", 0, Duration::from_millis(50)).is_none());
        m.record("check:result", json!({ "ok": true }));
        let e = m.wait_event("check:result", 1, Duration::from_millis(50)).unwrap();
        assert_eq!(e.seq, 2);
        assert!(m.wait_event("check:result", 2, Duration::from_millis(10)).is_none());
    }

    #[test]
    fn find_by_text_prefers_exact() {
        let js = find_js(&json!({ "text": "Start" })).unwrap();
        assert!(js.contains("=== \"start\""));
        assert!(find_js(&json!({})).is_err());
    }
}
