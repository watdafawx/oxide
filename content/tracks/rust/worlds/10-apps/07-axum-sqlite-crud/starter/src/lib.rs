use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

pub type Db = Arc<Mutex<Connection>>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Note {
    pub id: i64,
    pub body: String,
}

#[derive(Deserialize)]
pub struct NewNote {
    pub body: String,
}

/// An in-memory database with the `notes` table ready.
pub fn open_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE notes (id INTEGER PRIMARY KEY, body TEXT NOT NULL)",
        [],
    )
    .unwrap();
    conn
}

/// Any unexpected database error is a 500.
fn internal(_: rusqlite::Error) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

/// POST /notes {"body": "..."} -> 201 {"id": 1, "body": "..."}
async fn create(
    State(db): State<Db>,
    Json(new): Json<NewNote>,
) -> Result<(StatusCode, Json<Note>), StatusCode> {
    // TODO: lock db, INSERT INTO notes (body) VALUES (?1), id = conn.last_insert_rowid(), return Ok((StatusCode::CREATED, Json(Note { id, body: new.body })))
    todo!()
}

/// GET /notes -> [{"id":1,"body":".."}, ...] ordered by id.
async fn list(State(db): State<Db>) -> Result<Json<Vec<Note>>, StatusCode> {
    // TODO: lock db, prepare "SELECT id, body FROM notes ORDER BY id", query_map into Note, collect::<Result<Vec<_>, _>>().map_err(internal)?, wrap in Json
    todo!()
}

/// GET /notes/{id} -> the note, or 404.
async fn get_one(State(db): State<Db>, Path(id): Path<i64>) -> Result<Json<Note>, StatusCode> {
    // TODO: lock db, query_row(.. WHERE id = ?1 ..).optional().map_err(internal)?, then note.map(Json).ok_or(StatusCode::NOT_FOUND)
    todo!()
}

/// DELETE /notes/{id} -> 204, or 404 when nothing was deleted.
async fn remove(State(db): State<Db>, Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    // TODO: lock db, conn.execute("DELETE FROM notes WHERE id = ?1", params![id]) returns the row count; 0 -> Err(NOT_FOUND), else Ok(NO_CONTENT)
    todo!()
}

pub fn app(conn: Connection) -> Router {
    Router::new()
        .route("/notes", get(list).post(create))
        .route("/notes/{id}", get(get_one).delete(remove))
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
        let (status, json) = send(&app, "POST", "/notes", Some(r#"{"body":"buy milk"}"#)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(json["id"], 1);
        assert_eq!(json["body"], "buy milk");
    }

    #[tokio::test]
    async fn list_returns_notes_in_order() {
        let app = app(open_db());
        send(&app, "POST", "/notes", Some(r#"{"body":"first"}"#)).await;
        send(&app, "POST", "/notes", Some(r#"{"body":"second"}"#)).await;
        let (status, json) = send(&app, "GET", "/notes", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json.as_array().unwrap().len(), 2);
        assert_eq!(json[0]["body"], "first");
        assert_eq!(json[1]["id"], 2);
    }

    #[tokio::test]
    async fn get_missing_note_is_404() {
        let app = app(open_db());
        let (status, _) = send(&app, "GET", "/notes/99", None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        send(&app, "POST", "/notes", Some(r#"{"body":"here"}"#)).await;
        let (status, json) = send(&app, "GET", "/notes/1", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["body"], "here");
    }

    #[tokio::test]
    async fn delete_removes_the_note() {
        let app = app(open_db());
        send(&app, "POST", "/notes", Some(r#"{"body":"bye"}"#)).await;
        let (first, _) = send(&app, "DELETE", "/notes/1", None).await;
        assert_eq!(first, StatusCode::NO_CONTENT);
        let (second, _) = send(&app, "DELETE", "/notes/1", None).await;
        assert_eq!(second, StatusCode::NOT_FOUND);
        let (_, list) = send(&app, "GET", "/notes", None).await;
        assert!(list.as_array().unwrap().is_empty());
    }
}
