Each time a mod finishes downloading, the launcher sends `POST /downloads {"mb": 120}`, and the library screen asks `GET /downloads` for the running total. So far every handler forgot everything the moment it answered. Now the server has to remember one number across requests, and many requests can arrive at the same time.

**Shared state.** You hand the router a value once, with `.with_state(AppState::default())`, and any handler that asks for it with a `State(state): State<AppState>` parameter gets it. `State` is an extractor like `Path` and `Json`, and like them it goes before the `Json` body in the parameter list.

**Every request gets a clone.** axum clones the state for each request, so `AppState` must be `Clone`. A clone of a plain `u32` would be a separate number, and every download would vanish. Wrapping the counter in an `Arc`, from world 07, makes every clone point at the same counter.

**Many at once.** Requests are handled on several threads at the same time, so two downloads can finish in the same instant. From world 07: an **atomic** like `AtomicU32` is a number that many threads can change safely without a lock. `fetch_add(n, Ordering::SeqCst)` adds `n` and gives back the **old** value; `load(Ordering::SeqCst)` reads the current one.

```rust
#[derive(Clone, Default)]
struct Stats {
    logins: Arc<AtomicU32>,
}

async fn login(State(stats): State<Stats>) -> (StatusCode, Json<Logins>) {
    let logins = stats.logins.fetch_add(1, Ordering::SeqCst) + 1;
    (StatusCode::CREATED, Json(Logins { logins }))
}
// Router::new().route("/logins", post(login)).with_state(Stats::default())
```

**A different success.** A `POST` that added something answers `201 Created`, not 200. A pair `(StatusCode::CREATED, Json(..))` is a response, the same pair shape as last quest's error, now on the success side.

**One path, two methods.** `.route("/downloads", get(downloads).post(record_download))` puts both on one path: `GET` reads, `POST` adds. The starter has the routes and the state wired already.

## Your turn

- Give `record_download` and `downloads` the `State<AppState>` parameter.
- In `record_download`, add the megabytes to the counter and answer `201 Created` with the new total.
- In `downloads`, answer the total as it is now.
