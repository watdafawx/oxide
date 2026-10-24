use rusqlite::{Connection, Result, params};

/// The quests table, empty. The CHECK makes SQLite itself refuse a blank title.
pub fn open() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE quests (id INTEGER PRIMARY KEY, title TEXT NOT NULL CHECK (title <> ''), xp INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts every `(title, xp)` in one transaction and returns how many were added.
/// If any insert fails, nothing is kept.
pub fn add_world(conn: &mut Connection, quests: &[(&str, i64)]) -> Result<usize> {
    // TODO: start a transaction, insert every quest on it, commit it,
    //       then return how many quests there were
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM quests", [], |r| r.get(0))
            .unwrap()
    }

    #[test]
    fn add_world_inserts_every_quest() {
        let mut conn = open().unwrap();
        let added = add_world(&mut conn, &[("Variables", 40), ("Loops", 50), ("Boss", 300)]).unwrap();
        assert_eq!(added, 3);
        assert_eq!(count(&conn), 3);
    }

    #[test]
    fn a_blank_title_keeps_none_of_them() {
        let mut conn = open().unwrap();
        let result = add_world(&mut conn, &[("Variables", 40), ("", 50), ("Boss", 300)]);
        assert!(result.is_err(), "the blank title must fail");
        assert_eq!(count(&conn), 0, "'Variables' must be rolled back too");
    }
}
