use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

/// The one connection every handler shares.
pub type Db = Arc<Mutex<Connection>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Quest {
    pub id: i64,
    pub title: String,
    pub xp: i64,
}

/// The body of POST /quests.
#[derive(Deserialize)]
pub struct NewQuest {
    pub title: String,
    pub xp: i64,
}

/// An in-memory database with an empty quests table.
pub fn open_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE quests (id INTEGER PRIMARY KEY, title TEXT NOT NULL, xp INTEGER NOT NULL)",
        [],
    )
    .unwrap();
    conn
}

/// Any unexpected database error is the server's fault: 500.
fn internal(_: rusqlite::Error) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

/// POST /quests {"title": "Loops", "xp": 50} -> 201 {"id": 1, "title": "Loops", "xp": 50}
async fn create(
    State(db): State<Db>,
    Json(new): Json<NewQuest>,
) -> Result<(StatusCode, Json<Quest>), StatusCode> {
    // TODO: lock the database, insert the new quest (mapping any error with `internal`),
    //       and answer 201 Created with the stored Quest and its new id
    todo!()
}

/// GET /quests -> every quest, ordered by id.
async fn list(State(db): State<Db>) -> Result<Json<Vec<Quest>>, StatusCode> {
    // TODO: lock the database, read every quest ordered by id (prepare, query_map,
    //       collect, each with `internal` for errors), and answer them as JSON
    todo!()
}

pub fn app(conn: Connection) -> Router {
    Router::new()
        .route("/quests", get(list).post(create))
        .with_state(Arc::new(Mutex::new(conn)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt; // for `oneshot`

    async fn send(
        app: &Router,
        method: &str,
        uri: &str,
        body: Option<&str>,
    ) -> (StatusCode, serde_json::Value) {
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.unwrap_or("").to_string()))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        let status = res.status();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, json)
    }

    #[tokio::test]
    async fn create_returns_201_and_the_row() {
        let app = app(open_db());
        let (status, json) =
            send(&app, "POST", "/quests", Some(r#"{"title": "Loops", "xp": 50}"#)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(json, serde_json::json!({"id": 1, "title": "Loops", "xp": 50}));
        let (_, json) = send(&app, "POST", "/quests", Some(r#"{"title": "Boss", "xp": 300}"#)).await;
        assert_eq!(json["id"], 2);
    }

    #[tokio::test]
    async fn list_returns_quests_in_order() {
        let app = app(open_db());
        send(&app, "POST", "/quests", Some(r#"{"title": "Variables", "xp": 40}"#)).await;
        send(&app, "POST", "/quests", Some(r#"{"title": "Loops", "xp": 50}"#)).await;
        let (status, json) = send(&app, "GET", "/quests", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json.as_array().map(|a| a.len()), Some(2), "two quests, got {json}");
        assert_eq!(json[0]["title"], "Variables");
        assert_eq!(json[1]["id"], 2);
    }
}
