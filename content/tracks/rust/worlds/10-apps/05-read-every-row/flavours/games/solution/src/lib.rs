use rusqlite::{Connection, Result, params};

#[derive(Debug, PartialEq)]
pub struct InstalledMod {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
}

/// The mods table, empty.
pub fn open_mod_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts a mod and returns its id.
pub fn add_mod(conn: &Connection, name: &str, enabled: bool) -> Result<i64> {
    conn.execute(
        "INSERT INTO mods (name, enabled) VALUES (?1, ?2)",
        params![name, enabled],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Every mod, ordered by id.
pub fn list_mods(conn: &Connection) -> Result<Vec<InstalledMod>> {
    let mut stmt = conn.prepare("SELECT id, name, enabled FROM mods ORDER BY id")?;
    stmt.query_map([], |row| {
        Ok(InstalledMod {
            id: row.get(0)?,
            name: row.get(1)?,
            enabled: row.get(2)?,
        })
    })?
    .collect()
}

/// Every mod name, from A to Z.
pub fn a_to_z(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM mods ORDER BY name")?;
    stmt.query_map([], |row| row.get(0))?.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Connection {
        let conn = open_mod_db().unwrap();
        add_mod(&conn, "sodium", true).unwrap();
        add_mod(&conn, "iris", false).unwrap();
        add_mod(&conn, "lithium", true).unwrap();
        conn
    }

    #[test]
    fn list_mods_returns_every_row_in_id_order() {
        let conn = sample();
        assert_eq!(
            list_mods(&conn).unwrap(),
            vec![
                InstalledMod { id: 1, name: "sodium".into(), enabled: true },
                InstalledMod { id: 2, name: "iris".into(), enabled: false },
                InstalledMod { id: 3, name: "lithium".into(), enabled: true },
            ]
        );
        assert!(list_mods(&open_mod_db().unwrap()).unwrap().is_empty(), "an empty table gives an empty Vec");
    }

    #[test]
    fn a_to_z_sorts_names_alphabetically() {
        let conn = sample();
        assert_eq!(a_to_z(&conn).unwrap(), vec!["iris", "lithium", "sodium"]);
    }
}
