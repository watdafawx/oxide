/// "restart in 5" -> "[Server] restart in 5"
pub fn announce(message: &str) -> String {
    let mut line = String::from("[Server] ");
    line.push_str(message);
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn announce_gives_back_new_text() {
        assert_eq!(announce("restart in 5"), "[Server] restart in 5");
        assert_eq!(announce("gg"), "[Server] gg");
    }
}
