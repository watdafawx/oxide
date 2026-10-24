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
pub struct ModRow {
    pub id: i64,
    pub name: String,
}

/// The body of POST /mods.
#[derive(Deserialize)]
pub struct NewMod {
    pub name: String,
}

/// An in-memory database with an empty mods table.
pub fn open_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        [],
    )
    .unwrap();
    conn
}

/// Any unexpected database error is the server's fault: 500.
fn internal(_: rusqlite::Error) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

/// POST /mods {"name": "sodium"} -> 201 {"id": 1, "name": "sodium"}
async fn add_mod(
    State(db): State<Db>,
    Json(new): Json<NewMod>,
) -> Result<(StatusCode, Json<ModRow>), StatusCode> {
    // TODO: lock the database, insert the new mod (mapping any error with `internal`),
    //       and answer 201 Created with the stored ModRow and its new id
    todo!()
}

/// GET /mods -> every mod, ordered by id.
async fn list_mods(State(db): State<Db>) -> Result<Json<Vec<ModRow>>, StatusCode> {
    // TODO: lock the database, read every mod ordered by id (prepare, query_map,
    //       collect, each with `internal` for errors), and answer them as JSON
    todo!()
}

pub fn app(conn: Connection) -> Router {
    Router::new()
        .route("/mods", get(list_mods).post(add_mod))
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
    async fn adding_a_mod_returns_201_and_the_row() {
        let app = app(open_db());
        let (status, json) = send(&app, "POST", "/mods", Some(r#"{"name": "sodium"}"#)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(json, serde_json::json!({"id": 1, "name": "sodium"}));
        let (_, json) = send(&app, "POST", "/mods", Some(r#"{"name": "iris"}"#)).await;
        assert_eq!(json["id"], 2);
    }

    #[tokio::test]
    async fn listing_returns_mods_in_order() {
        let app = app(open_db());
        send(&app, "POST", "/mods", Some(r#"{"name": "sodium"}"#)).await;
        send(&app, "POST", "/mods", Some(r#"{"name": "lithium"}"#)).await;
        let (status, json) = send(&app, "GET", "/mods", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json.as_array().map(|a| a.len()), Some(2), "two mods, got {json}");
        assert_eq!(json[0]["name"], "sodium");
        assert_eq!(json[1]["id"], 2);
    }
}
