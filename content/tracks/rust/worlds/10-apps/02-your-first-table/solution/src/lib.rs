use rusqlite::{Connection, Result};

/// Opens a database in memory with an empty `quests` table:
/// `CREATE TABLE quests (id INTEGER PRIMARY KEY, title TEXT NOT NULL, xp INTEGER NOT NULL)`
pub fn open() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE quests (id INTEGER PRIMARY KEY, title TEXT NOT NULL, xp INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The column names of the quests table, in order.
    fn columns(conn: &Connection) -> Vec<String> {
        let mut stmt = conn
            .prepare("SELECT name FROM pragma_table_info('quests')")
            .unwrap();
        stmt.query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_>>()
            .unwrap()
    }

    #[test]
    fn open_creates_the_quests_table() {
        let conn = open().unwrap();
        assert_eq!(columns(&conn), vec!["id", "title", "xp"]);
        let rows: i64 = conn
            .query_row("SELECT COUNT(*) FROM quests", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rows, 0, "a new table has no rows yet");
    }
}
