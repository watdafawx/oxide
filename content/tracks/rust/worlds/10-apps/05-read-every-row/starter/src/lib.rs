use rusqlite::{Connection, Result, params};

#[derive(Debug, PartialEq)]
pub struct QuestRow {
    pub id: i64,
    pub title: String,
    pub xp: i64,
}

/// The quests table, empty.
pub fn open() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE quests (id INTEGER PRIMARY KEY, title TEXT NOT NULL, xp INTEGER NOT NULL)",
        [],
    )?;
    Ok(conn)
}

/// Inserts a quest and returns its id.
pub fn add_quest(conn: &Connection, title: &str, xp: i64) -> Result<i64> {
    conn.execute(
        "INSERT INTO quests (title, xp) VALUES (?1, ?2)",
        params![title, xp],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Every quest, ordered by id.
pub fn list_quests(conn: &Connection) -> Result<Vec<QuestRow>> {
    // TODO: prepare a SELECT of id, title and xp ordered by id, map each row into a
    //       QuestRow, and collect the rows
    todo!()
}

/// Every title, the quest with the most XP first.
pub fn biggest_first(conn: &Connection) -> Result<Vec<String>> {
    // TODO: the same three steps, selecting only the title and sorting by xp, biggest first
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Connection {
        let conn = open().unwrap();
        add_quest(&conn, "Loops", 50).unwrap();
        add_quest(&conn, "Boss: crafting calculator", 300).unwrap();
        add_quest(&conn, "Variables", 40).unwrap();
        conn
    }

    #[test]
    fn list_quests_returns_every_row_in_id_order() {
        let conn = sample();
        assert_eq!(
            list_quests(&conn).unwrap(),
            vec![
                QuestRow { id: 1, title: "Loops".into(), xp: 50 },
                QuestRow { id: 2, title: "Boss: crafting calculator".into(), xp: 300 },
                QuestRow { id: 3, title: "Variables".into(), xp: 40 },
            ]
        );
        assert!(list_quests(&open().unwrap()).unwrap().is_empty(), "an empty table gives an empty Vec");
    }

    #[test]
    fn biggest_first_sorts_titles_by_xp() {
        let conn = sample();
        assert_eq!(
            biggest_first(&conn).unwrap(),
            vec!["Boss: crafting calculator", "Loops", "Variables"]
        );
    }
}
