use rusqlite::{Connection, Result, params};

/// The schema version stored in `PRAGMA user_version` (0 for a brand-new database).
pub fn version(conn: &Connection) -> Result<i64> {
    conn.pragma_query_value(None, "user_version", |r| r.get(0))
}

/// Brings any database up to version 2, inside one transaction.
/// v1: `notes (id INTEGER PRIMARY KEY, body TEXT NOT NULL CHECK (body <> ''))`
/// v2: `ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0`
pub fn migrate(conn: &mut Connection) -> Result<()> {
    // TODO: start a transaction, read version(&tx), run only the missing steps (v1, v2), tx.pragma_update(None, "user_version", 2)?, commit
    todo!()
}

/// Inserts every body in a single transaction. Returns how many were added.
/// If any insert fails nothing is kept.
pub fn add_many(conn: &mut Connection, bodies: &[&str]) -> Result<usize> {
    // TODO: transaction, prepare the INSERT once, execute it per body, commit, return bodies.len()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM notes", [], |r| r.get(0))
            .unwrap()
    }

    #[test]
    fn migrate_builds_latest_schema() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        assert_eq!(version(&conn).unwrap(), 2);
        conn.execute("INSERT INTO notes (body, pinned) VALUES ('hi', 1)", [])
            .unwrap();
        // migrating again must change nothing and not fail
        migrate(&mut conn).unwrap();
        assert_eq!(version(&conn).unwrap(), 2);
        assert_eq!(count(&conn), 1);
    }

    #[test]
    fn migrate_upgrades_old_db_and_keeps_rows() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, body TEXT NOT NULL CHECK (body <> ''));
             INSERT INTO notes (body) VALUES ('old note');
             PRAGMA user_version = 1;",
        )
        .unwrap();
        migrate(&mut conn).unwrap();
        assert_eq!(version(&conn).unwrap(), 2);
        let (body, pinned): (String, i64) = conn
            .query_row("SELECT body, pinned FROM notes", [], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .unwrap();
        assert_eq!(body, "old note");
        assert_eq!(pinned, 0, "old rows get the column default");
    }

    #[test]
    fn add_many_inserts_all_bodies() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        let n = add_many(&mut conn, &["a", "b", "c"]).unwrap();
        assert_eq!(n, 3);
        assert_eq!(count(&conn), 3);
    }

    #[test]
    fn add_many_rolls_back_on_error() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        // the empty body breaks the CHECK constraint
        let res = add_many(&mut conn, &["fine", "", "never reached"]);
        assert!(res.is_err(), "an empty body must fail");
        assert_eq!(count(&conn), 0, "'fine' must be rolled back too");
    }
}
