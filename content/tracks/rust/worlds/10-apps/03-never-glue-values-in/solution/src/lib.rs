use rusqlite::{Connection, Result, params};

/// The quests table, empty (last quest's `open`).
pub fn open() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE quests (id INTEGER PRIMARY KEY, title TEXT NOT NULL, xp INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts a quest and returns the id the database gave it.
pub fn add_quest(conn: &Connection, title: &str, xp: i64) -> Result<i64> {
    conn.execute(
        "INSERT INTO quests (title, xp) VALUES (?1, ?2)",
        params![title, xp],
    )?;
    Ok(conn.last_insert_rowid())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn title_of(conn: &Connection, id: i64) -> String {
        conn.query_row("SELECT title FROM quests WHERE id = ?1", [id], |r| r.get(0))
            .unwrap()
    }

    fn count(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM quests", [], |r| r.get(0))
            .unwrap()
    }

    #[test]
    fn a_title_with_an_apostrophe_is_stored_as_typed() {
        let conn = open().unwrap();
        let first = add_quest(&conn, "Variables", 40).unwrap();
        let second = add_quest(&conn, "Rust's traits", 60).unwrap();
        assert_eq!((first, second), (1, 2), "ids are handed out in order");
        assert_eq!(title_of(&conn, second), "Rust's traits");
    }

    #[test]
    fn a_title_that_looks_like_sql_is_only_text() {
        let conn = open().unwrap();
        add_quest(&conn, "Variables", 40).unwrap();
        let sneaky = "x', 0); DELETE FROM quests; --";
        let id = add_quest(&conn, sneaky, 10).unwrap();
        assert_eq!(title_of(&conn, id), sneaky, "the title is stored letter for letter");
        assert_eq!(count(&conn), 2, "nothing was deleted");
    }
}
