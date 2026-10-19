use rusqlite::{Connection, Result, params};

#[derive(Debug, PartialEq)]
pub struct InstalledMod {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
}

/// Opens an in-memory mod database with an empty `mods` table:
/// `mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)`.
pub fn open_mod_db() -> Result<Connection> {
    // TODO: Connection::open_in_memory(), then CREATE TABLE mods (...)
    todo!()
}

/// Inserts an enabled mod and returns its new id.
pub fn add_mod(conn: &Connection, name: &str) -> Result<i64> {
    // TODO: INSERT with params![name], then last_insert_rowid()
    todo!()
}

/// Every mod, ordered by id.
pub fn list_mods(conn: &Connection) -> Result<Vec<InstalledMod>> {
    // TODO: prepare a SELECT, query_map each row into an InstalledMod, collect
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_mod_db_creates_the_mods_table() {
        let conn = open_mod_db().unwrap();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM mods", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 0, "the mods table should exist and be empty");
    }

    #[test]
    fn add_mod_hands_out_ids_in_order() {
        let conn = open_mod_db().unwrap();
        let a = add_mod(&conn, "sodium").unwrap();
        let b = add_mod(&conn, "lithium").unwrap();
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }

    #[test]
    fn list_mods_returns_rows_in_id_order() {
        let conn = open_mod_db().unwrap();
        add_mod(&conn, "sodium").unwrap();
        add_mod(&conn, "lithium").unwrap();
        let mods = list_mods(&conn).unwrap();
        assert_eq!(
            mods,
            vec![
                InstalledMod { id: 1, name: "sodium".into(), enabled: true },
                InstalledMod { id: 2, name: "lithium".into(), enabled: true },
            ]
        );
    }
}
