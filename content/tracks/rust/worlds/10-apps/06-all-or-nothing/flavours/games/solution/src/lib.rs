use rusqlite::{Connection, Result, params};

/// The mods table, empty. The CHECK makes SQLite itself refuse a blank name.
pub fn open_mod_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL CHECK (name <> ''), enabled INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts every mod, enabled, in one transaction and returns how many were added.
/// If any insert fails, nothing is kept.
pub fn install_pack(conn: &mut Connection, names: &[&str]) -> Result<usize> {
    let tx = conn.transaction()?;
    for name in names {
        tx.execute(
            "INSERT INTO mods (name, enabled) VALUES (?1, 1)",
            params![name],
        )?;
    }
    tx.commit()?;
    Ok(names.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM mods", [], |r| r.get(0))
            .unwrap()
    }

    #[test]
    fn install_pack_inserts_every_mod() {
        let mut conn = open_mod_db().unwrap();
        let added = install_pack(&mut conn, &["sodium", "lithium", "iris"]).unwrap();
        assert_eq!(added, 3);
        assert_eq!(count(&conn), 3);
    }

    #[test]
    fn a_blank_name_keeps_none_of_them() {
        let mut conn = open_mod_db().unwrap();
        let result = install_pack(&mut conn, &["sodium", "", "iris"]);
        assert!(result.is_err(), "the blank name must fail");
        assert_eq!(count(&conn), 0, "'sodium' must be rolled back too");
    }
}
