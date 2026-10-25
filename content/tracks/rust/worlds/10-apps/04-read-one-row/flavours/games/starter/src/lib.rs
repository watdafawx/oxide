use rusqlite::{Connection, Result, params};

/// The mods table, empty.
pub fn open_mod_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts a mod and returns its id (last quest's `add_mod`).
pub fn add_mod(conn: &Connection, name: &str, enabled: bool) -> Result<i64> {
    conn.execute(
        "INSERT INTO mods (name, enabled) VALUES (?1, ?2)",
        params![name, enabled],
    )?;
    Ok(conn.last_insert_rowid())
}

/// The name of the mod with this id. An id with no row is an `Err`.
pub fn name_of(conn: &Connection, id: i64) -> Result<String> {
    // TODO: ask for the name of the one mod whose id is ?1, and read column 0
    todo!()
}

/// How many mods are enabled (`true`) or disabled (`false`).
pub fn count_by_state(conn: &Connection, enabled: bool) -> Result<i64> {
    // TODO: let SQLite count the mods whose enabled column is ?1, and read that one number
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Connection {
        let conn = open_mod_db().unwrap();
        add_mod(&conn, "sodium", true).unwrap();
        add_mod(&conn, "lithium", true).unwrap();
        add_mod(&conn, "optifine", false).unwrap();
        conn
    }

    #[test]
    fn name_of_finds_the_row() {
        let conn = sample();
        assert_eq!(name_of(&conn, 2).unwrap(), "lithium");
        assert_eq!(name_of(&conn, 1).unwrap(), "sodium");
        assert!(name_of(&conn, 99).is_err(), "there is no mod 99");
    }

    #[test]
    fn count_by_state_counts_matching_rows() {
        let conn = sample();
        assert_eq!(count_by_state(&conn, true).unwrap(), 2);
        assert_eq!(count_by_state(&conn, false).unwrap(), 1);
    }
}
