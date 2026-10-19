use rusqlite::{Connection, Result, params};

/// The schema version stored in `PRAGMA user_version` (0 for a brand-new database).
pub fn version(conn: &Connection) -> Result<i64> {
    conn.pragma_query_value(None, "user_version", |r| r.get(0))
}

/// Brings any database up to version 2, inside one transaction.
/// v1: `mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL CHECK (name <> ''))`
/// v2: `ALTER TABLE mods ADD COLUMN favourite INTEGER NOT NULL DEFAULT 0`
pub fn migrate(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    let v = version(&tx)?;
    if v < 1 {
        tx.execute_batch(
            "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL CHECK (name <> ''))",
        )?;
    }
    if v < 2 {
        tx.execute_batch("ALTER TABLE mods ADD COLUMN favourite INTEGER NOT NULL DEFAULT 0")?;
    }
    tx.pragma_update(None, "user_version", 2)?;
    tx.commit()
}

/// Inserts every name in a single transaction. Returns how many were added.
/// If any insert fails nothing is kept.
pub fn add_mods(conn: &mut Connection, names: &[&str]) -> Result<usize> {
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare("INSERT INTO mods (name) VALUES (?1)")?;
        for name in names {
            stmt.execute(params![name])?;
        }
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
    fn migrate_builds_the_latest_mod_schema() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        assert_eq!(version(&conn).unwrap(), 2);
        conn.execute("INSERT INTO mods (name, favourite) VALUES ('sodium', 1)", [])
            .unwrap();
        // migrating again must change nothing and not fail
        migrate(&mut conn).unwrap();
        assert_eq!(version(&conn).unwrap(), 2);
        assert_eq!(count(&conn), 1);
    }

    #[test]
    fn migrate_upgrades_an_old_db_and_keeps_mods() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL CHECK (name <> ''));
             INSERT INTO mods (name) VALUES ('old mod');
             PRAGMA user_version = 1;",
        )
        .unwrap();
        migrate(&mut conn).unwrap();
        assert_eq!(version(&conn).unwrap(), 2);
        let (name, favourite): (String, i64) = conn
            .query_row("SELECT name, favourite FROM mods", [], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .unwrap();
        assert_eq!(name, "old mod");
        assert_eq!(favourite, 0, "old rows get the column default");
    }

    #[test]
    fn add_mods_inserts_every_name() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        let n = add_mods(&mut conn, &["sodium", "lithium", "iris"]).unwrap();
        assert_eq!(n, 3);
        assert_eq!(count(&conn), 3);
    }

    #[test]
    fn add_mods_rolls_back_on_a_bad_name() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        // the empty name breaks the CHECK constraint
        let res = add_mods(&mut conn, &["sodium", "", "never reached"]);
        assert!(res.is_err(), "an empty name must fail");
        assert_eq!(count(&conn), 0, "'sodium' must be rolled back too");
    }
}
