use rusqlite::{Connection, Result};

/// The mods table, empty (last quest's `open_mod_db`).
pub fn open_mod_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts a mod and returns the id the database gave it.
pub fn add_mod(conn: &Connection, name: &str, enabled: bool) -> Result<i64> {
    // TODO: this glues the values into the SQL text, so a ' in the name breaks it.
    //       Write ?1 and ?2 where the values go, and pass name and enabled separately.
    let sql = format!(
        "INSERT INTO mods (name, enabled) VALUES ('{name}', {})",
        enabled as i32
    );
    conn.execute(&sql, [])?;
    Ok(conn.last_insert_rowid())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name_of(conn: &Connection, id: i64) -> String {
        conn.query_row("SELECT name FROM mods WHERE id = ?1", [id], |r| r.get(0))
            .unwrap()
    }

    fn count(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM mods", [], |r| r.get(0))
            .unwrap()
    }

    #[test]
    fn a_name_with_an_apostrophe_is_stored_as_typed() {
        let conn = open_mod_db().unwrap();
        let first = add_mod(&conn, "sodium", true).unwrap();
        let second = add_mod(&conn, "Xaero's Minimap", true).unwrap();
        assert_eq!((first, second), (1, 2), "ids are handed out in order");
        assert_eq!(name_of(&conn, second), "Xaero's Minimap");
    }

    #[test]
    fn a_name_that_looks_like_sql_is_only_text() {
        let conn = open_mod_db().unwrap();
        add_mod(&conn, "sodium", true).unwrap();
        let sneaky = "x', 1); DELETE FROM mods; --";
        let id = add_mod(&conn, sneaky, false).unwrap();
        assert_eq!(name_of(&conn, id), sneaky, "the name is stored letter for letter");
        assert_eq!(count(&conn), 2, "nothing was deleted");
    }
}
