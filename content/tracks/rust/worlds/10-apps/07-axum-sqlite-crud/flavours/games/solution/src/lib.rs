use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

pub type Db = Arc<Mutex<Connection>>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ModRow {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewMod {
    pub name: String,
}

/// An in-memory database with the `mods` table ready.
pub fn open_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        [],
    )
    .unwrap();
    conn
}

/// Any unexpected database error is a 500.
fn internal(_: rusqlite::Error) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

/// POST /mods {"name": "..."} -> 201 {"id": 1, "name": "..."}
async fn add_mod(
    State(db): State<Db>,
    Json(new): Json<NewMod>,
) -> Result<(StatusCode, Json<ModRow>), StatusCode> {
    let conn = db.lock().unwrap();
    conn.execute("INSERT INTO mods (name) VALUES (?1)", params![new.name])
        .map_err(internal)?;
    let id = conn.last_insert_rowid();
    Ok((StatusCode::CREATED, Json(ModRow { id, name: new.name })))
}

/// GET /mods -> [{"id":1,"name":".."}, ...] ordered by id.
async fn list_mods(State(db): State<Db>) -> Result<Json<Vec<ModRow>>, StatusCode> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name FROM mods ORDER BY id")
        .map_err(internal)?;
    let mods = stmt
        .query_map([], |r| {
            Ok(ModRow {
                id: r.get(0)?,
                name: r.get(1)?,
            })
        })
        .map_err(internal)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(internal)?;
    Ok(Json(mods))
}

/// GET /mods/{id} -> the mod, or 404.
async fn get_mod(State(db): State<Db>, Path(id): Path<i64>) -> Result<Json<ModRow>, StatusCode> {
    let conn = db.lock().unwrap();
    let found = conn
        .query_row(
            "SELECT id, name FROM mods WHERE id = ?1",
            params![id],
            |r| {
                Ok(ModRow {
                    id: r.get(0)?,
                    name: r.get(1)?,
                })
            },
        )
        .optional()
        .map_err(internal)?;
    found.map(Json).ok_or(StatusCode::NOT_FOUND)
}

/// DELETE /mods/{id} -> 204, or 404 when nothing was deleted.
async fn remove_mod(State(db): State<Db>, Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    let conn = db.lock().unwrap();
    let deleted = conn
        .execute("DELETE FROM mods WHERE id = ?1", params![id])
        .map_err(internal)?;
    if deleted == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

pub fn app(conn: Connection) -> Router {
    Router::new()
        .route("/mods", get(list_mods).post(add_mod))
        .route("/mods/{id}", get(get_mod).delete(remove_mod))
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
        let (status, json) = send(&app, "POST", "/mods", Some(r#"{"name":"sodium"}"#)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(json["id"], 1);
        assert_eq!(json["name"], "sodium");
    }

    #[tokio::test]
    async fn listing_returns_mods_in_order() {
        let app = app(open_db());
        send(&app, "POST", "/mods", Some(r#"{"name":"sodium"}"#)).await;
        send(&app, "POST", "/mods", Some(r#"{"name":"lithium"}"#)).await;
        let (status, json) = send(&app, "GET", "/mods", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json.as_array().unwrap().len(), 2);
        assert_eq!(json[0]["name"], "sodium");
        assert_eq!(json[1]["id"], 2);
    }

    #[tokio::test]
    async fn fetching_a_missing_mod_is_404() {
        let app = app(open_db());
        let (status, _) = send(&app, "GET", "/mods/99", None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        send(&app, "POST", "/mods", Some(r#"{"name":"iris"}"#)).await;
        let (status, json) = send(&app, "GET", "/mods/1", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["name"], "iris");
    }

    #[tokio::test]
    async fn deleting_removes_the_mod() {
        let app = app(open_db());
        send(&app, "POST", "/mods", Some(r#"{"name":"jei"}"#)).await;
        let (first, _) = send(&app, "DELETE", "/mods/1", None).await;
        assert_eq!(first, StatusCode::NO_CONTENT);
        let (second, _) = send(&app, "DELETE", "/mods/1", None).await;
        assert_eq!(second, StatusCode::NOT_FOUND);
        let (_, list) = send(&app, "GET", "/mods", None).await;
        assert!(list.as_array().unwrap().is_empty());
    }
}
