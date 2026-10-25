use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, params};
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
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO quests (title, xp) VALUES (?1, ?2)",
        params![new.title, new.xp],
    )
    .map_err(internal)?;
    let id = conn.last_insert_rowid();
    Ok((
        StatusCode::CREATED,
        Json(Quest {
            id,
            title: new.title,
            xp: new.xp,
        }),
    ))
}

/// GET /quests -> every quest, ordered by id.
async fn list(State(db): State<Db>) -> Result<Json<Vec<Quest>>, StatusCode> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, title, xp FROM quests ORDER BY id")
        .map_err(internal)?;
    let quests = stmt
        .query_map([], |r| {
            Ok(Quest {
                id: r.get(0)?,
                title: r.get(1)?,
                xp: r.get(2)?,
            })
        })
        .map_err(internal)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(internal)?;
    Ok(Json(quests))
}

/// GET /quests/{id} -> the quest, or 404.
async fn get_one(State(db): State<Db>, Path(id): Path<i64>) -> Result<Json<Quest>, StatusCode> {
    // TODO: lock the database, read the quest with this id as an Option (so "no row" is
    //       not an error), and answer it as JSON, or 404 Not Found when there is none
    todo!()
}

/// DELETE /quests/{id} -> 204, or 404 when there was nothing to delete.
async fn remove(State(db): State<Db>, Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    // TODO: lock the database, delete the quest with this id, and answer 204 No Content,
    //       or 404 Not Found when no row was deleted
    todo!()
}

pub fn app(conn: Connection) -> Router {
    Router::new()
        .route("/quests", get(list).post(create))
        .route("/quests/{id}", get(get_one).delete(remove))
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

    async fn add(app: &Router, title: &str) {
        let body = format!(r#"{{"title": "{title}", "xp": 50}}"#);
        send(app, "POST", "/quests", Some(&body)).await;
    }

    #[tokio::test]
    async fn get_missing_quest_is_404() {
        let app = app(open_db());
        let (status, _) = send(&app, "GET", "/quests/99", None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        add(&app, "Loops").await;
        let (status, json) = send(&app, "GET", "/quests/1", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["title"], "Loops");
    }

    #[tokio::test]
    async fn delete_answers_204_once_then_404() {
        let app = app(open_db());
        add(&app, "Variables").await;
        let (first, _) = send(&app, "DELETE", "/quests/1", None).await;
        assert_eq!(first, StatusCode::NO_CONTENT);
        let (second, _) = send(&app, "DELETE", "/quests/1", None).await;
        assert_eq!(second, StatusCode::NOT_FOUND);
        let (_, list) = send(&app, "GET", "/quests", None).await;
        assert_eq!(list, serde_json::json!([]), "the quest is gone from the list");
    }
}
