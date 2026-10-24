pub struct Server {
    pub xp: Vec<u32>,
    pub names: Vec<String>,
}

/// Double-xp weekend: every player gets 10 xp.
pub fn boost_all(server: &mut Server) {
    // TODO: lend each xp value mutably and add 10 to it
    todo!()
}

/// Adds " [AFK]" to the end of every player name.
pub fn tag_all_afk(server: &mut Server) {
    // TODO: lend each name mutably and add " [AFK]" to it
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn server() -> Server {
        Server {
            xp: vec![0, 45, 100],
            names: vec!["Steve".to_string(), "Alex".to_string()],
        }
    }

    #[test]
    fn boost_all_adds_xp_to_everyone() {
        let mut s = server();
        boost_all(&mut s);
        assert_eq!(s.xp, vec![10, 55, 110]);
    }

    #[test]
    fn tag_all_afk_marks_every_name() {
        let mut s = server();
        tag_all_afk(&mut s);
        assert_eq!(s.names, vec!["Steve [AFK]", "Alex [AFK]"]);
    }
}
