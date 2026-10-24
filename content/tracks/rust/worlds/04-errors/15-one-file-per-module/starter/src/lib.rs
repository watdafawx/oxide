// TODO: load src/parser.rs and src/report.rs as public modules

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_file_is_wired_in() {
        assert_eq!(parser::priority_of("3|buy milk"), 3);
        assert_eq!(parser::priority_of("no priority here"), 0);
    }

    #[test]
    fn report_adds_up_through_the_parser() {
        assert_eq!(report::total_priority(&["3|buy milk", "1|call mum", "oops"]), 4);
        assert_eq!(report::total_priority(&[]), 0);
    }
}
