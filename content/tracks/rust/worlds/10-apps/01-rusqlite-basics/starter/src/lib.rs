use rusqlite::{Connection, Result, params};

#[derive(Debug, PartialEq)]
pub struct Mod {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
}

/// Opens an in-memory database with an empty `mods` table:
/// `mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)`. Given.
pub fn open() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts an enabled mod and returns its new id.
pub fn add_mod(conn: &Connection, name: &str) -> Result<i64> {
    // TODO: run an INSERT that puts `name` in through a ?1 placeholder (enabled = 1), then return the new row's id
    todo!()
}

/// Every mod, ordered by id.
pub fn list_mods(conn: &Connection) -> Result<Vec<Mod>> {
    // TODO: prepare a SELECT of id, name, enabled ordered by id, turn each row into a Mod
    //       with query_map, then collect the rows
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_creates_mods_table() {
        let conn = open().unwrap();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM mods", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 0, "the mods table should exist and be empty");
    }

    #[test]
    fn add_mod_returns_new_id() {
        let conn = open().unwrap();
        let a = add_mod(&conn, "sodium").unwrap();
        let b = add_mod(&conn, "lithium").unwrap();
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }

    #[test]
    fn list_mods_returns_rows_in_order() {
        let conn = open().unwrap();
        add_mod(&conn, "sodium").unwrap();
        add_mod(&conn, "lithium").unwrap();
        let mods = list_mods(&conn).unwrap();
        assert_eq!(
            mods,
            vec![
                Mod { id: 1, name: "sodium".into(), enabled: true },
                Mod { id: 2, name: "lithium".into(), enabled: true },
            ]
        );
    }
}
