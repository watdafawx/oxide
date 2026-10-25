use rusqlite::{Connection, Result};

/// Opens a database in memory with an empty `mods` table:
/// `CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)`
pub fn open_mod_db() -> Result<Connection> {
    // TODO: open a database in memory, run the CREATE TABLE statement above on it,
    //       and hand the connection back
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The column names of the mods table, in order.
    fn columns(conn: &Connection) -> Vec<String> {
        let mut stmt = conn
            .prepare("SELECT name FROM pragma_table_info('mods')")
            .unwrap();
        stmt.query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_>>()
            .unwrap()
    }

    #[test]
    fn open_mod_db_creates_the_mods_table() {
        let conn = open_mod_db().unwrap();
        assert_eq!(columns(&conn), vec!["id", "name", "enabled"]);
        let rows: i64 = conn
            .query_row("SELECT COUNT(*) FROM mods", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rows, 0, "a new table has no rows yet");
    }
}
