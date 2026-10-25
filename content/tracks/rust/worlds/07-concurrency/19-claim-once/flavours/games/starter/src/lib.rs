use std::sync::atomic::{AtomicBool, Ordering};

/// Claims the job of writing the crash report for a broken jar. Returns true only for the one
/// caller that flipped `flag` from false to true; everyone after that gets false.
pub fn claim_report(flag: &AtomicBool) -> bool {
    // TODO: compare_exchange from false to true (Ordering::SeqCst twice), then is_ok()
    !flag.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::thread;

    #[test]
    fn claim_report_has_exactly_one_winner() {
        let flag = AtomicBool::new(false);
        assert!(claim_report(&flag), "the first caller wins");
        assert!(!claim_report(&flag), "the second caller must lose");

        for _ in 0..20 {
            let flag = AtomicBool::new(false);
            let winners = AtomicUsize::new(0);
            thread::scope(|s| {
                for _ in 0..8 {
                    s.spawn(|| {
                        if claim_report(&flag) {
                            winners.fetch_add(1, Ordering::SeqCst);
                        }
                    });
                }
            });
            assert_eq!(winners.load(Ordering::SeqCst), 1, "exactly one of 8 workers may win");
        }
    }
}
