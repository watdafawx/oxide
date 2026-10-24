use rusqlite::{Connection, Result, params};

/// The quests table, empty.
pub fn open() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE quests (id INTEGER PRIMARY KEY, title TEXT NOT NULL, xp INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts a quest and returns its id (last quest's `add_quest`).
pub fn add_quest(conn: &Connection, title: &str, xp: i64) -> Result<i64> {
    conn.execute(
        "INSERT INTO quests (title, xp) VALUES (?1, ?2)",
        params![title, xp],
    )?;
    Ok(conn.last_insert_rowid())
}

/// The title of the quest with this id. An id with no row is an `Err`.
pub fn title_of(conn: &Connection, id: i64) -> Result<String> {
    // TODO: ask for the title of the one quest whose id is ?1, and read column 0
    todo!()
}

/// How many quests are worth `min_xp` XP or more.
pub fn count_worth_at_least(conn: &Connection, min_xp: i64) -> Result<i64> {
    // TODO: let SQLite count the quests whose xp is at least ?1, and read that one number
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Connection {
        let conn = open().unwrap();
        add_quest(&conn, "Variables", 40).unwrap();
        add_quest(&conn, "Loops", 50).unwrap();
        add_quest(&conn, "Boss: crafting calculator", 300).unwrap();
        conn
    }

    #[test]
    fn title_of_finds_the_row() {
        let conn = sample();
        assert_eq!(title_of(&conn, 2).unwrap(), "Loops");
        assert_eq!(title_of(&conn, 1).unwrap(), "Variables");
        assert!(title_of(&conn, 99).is_err(), "there is no quest 99");
    }

    #[test]
    fn count_worth_at_least_counts_matching_rows() {
        let conn = sample();
        assert_eq!(count_worth_at_least(&conn, 50).unwrap(), 2);
        assert_eq!(count_worth_at_least(&conn, 0).unwrap(), 3);
        assert_eq!(count_worth_at_least(&conn, 1000).unwrap(), 0);
    }
}
