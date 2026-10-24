Each time you finish a quest the screen sends `POST /xp {"amount": 50}`, and the header asks `GET /xp` for the running total. So far every handler forgot everything the moment it answered. Now the server has to remember one number across requests, and many requests can arrive at the same time.

**Shared state.** You hand the router a value once, with `.with_state(AppState::default())`, and any handler that asks for it with a `State(state): State<AppState>` parameter gets it. `State` is an extractor like `Path` and `Json`, and like them it goes before the `Json` body in the parameter list.

**Every request gets a clone.** axum clones the state for each request, so `AppState` must be `Clone`. A clone of a plain `u32` would be a separate number, and every award would vanish. Wrapping the counter in an `Arc`, from world 07, makes every clone point at the same counter.

**Many at once.** Requests are handled on several threads at the same time, so two awards can arrive in the same instant. From world 07: an **atomic** like `AtomicU32` is a number that many threads can change safely without a lock. `fetch_add(n, Ordering::SeqCst)` adds `n` and gives back the **old** value; `load(Ordering::SeqCst)` reads the current one.

```rust
#[derive(Clone, Default)]
struct Stats {
    visits: Arc<AtomicU32>,
}

async fn visit(State(stats): State<Stats>) -> (StatusCode, Json<Visits>) {
    let visits = stats.visits.fetch_add(1, Ordering::SeqCst) + 1;
    (StatusCode::CREATED, Json(Visits { visits }))
}
// Router::new().route("/visits", post(visit)).with_state(Stats::default())
```

**A different success.** A `POST` that added something answers `201 Created`, not 200. A pair `(StatusCode::CREATED, Json(..))` is a response, the same pair shape as last quest's error, now on the success side.

**One path, two methods.** `.route("/xp", get(current).post(award))` puts both on one path: `GET` reads, `POST` adds. The starter has the routes and the state wired already.

## Your turn

- Give `award` and `current` the `State<AppState>` parameter.
- In `award`, add the amount to the counter and answer `201 Created` with the new total.
- In `current`, answer the total as it is now.
