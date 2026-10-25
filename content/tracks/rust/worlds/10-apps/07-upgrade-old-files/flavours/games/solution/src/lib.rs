use rusqlite::{Connection, Result};

/// The schema version stored in the file (`PRAGMA user_version`); 0 for a new database.
pub fn version(conn: &Connection) -> Result<i64> {
    conn.pragma_query_value(None, "user_version", |r| r.get(0))
}

/// Brings any database up to version 2, inside one transaction.
/// v1: `CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)`
/// v2: `ALTER TABLE mods ADD COLUMN favourite INTEGER NOT NULL DEFAULT 0`
pub fn migrate(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    let v = version(&tx)?;
    if v < 1 {
        tx.execute(
            "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)",
            [],
        )?;
    }
    if v < 2 {
        tx.execute(
            "ALTER TABLE mods ADD COLUMN favourite INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    tx.pragma_update(None, "user_version", 2)?;
    tx.commit()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM mods", [], |r| r.get(0))
            .unwrap()
    }

    #[test]
    fn migrate_builds_the_latest_mod_schema() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        assert_eq!(version(&conn).unwrap(), 2);
        conn.execute("INSERT INTO mods (name, enabled, favourite) VALUES ('sodium', 1, 1)", [])
            .unwrap();
        migrate(&mut conn).expect("running migrate a second time must not fail");
        assert_eq!(version(&conn).unwrap(), 2);
        assert_eq!(count(&conn), 1);
    }

    #[test]
    fn migrate_upgrades_an_old_file_and_keeps_mods() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL);
             INSERT INTO mods (name, enabled) VALUES ('sodium', 1);
             PRAGMA user_version = 1;",
        )
        .unwrap();
        migrate(&mut conn).unwrap();
        assert_eq!(version(&conn).unwrap(), 2);
        let (name, favourite): (String, i64) = conn
            .query_row("SELECT name, favourite FROM mods", [], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap();
        assert_eq!(name, "sodium");
        assert_eq!(favourite, 0, "old rows get the column's default");
    }
}
