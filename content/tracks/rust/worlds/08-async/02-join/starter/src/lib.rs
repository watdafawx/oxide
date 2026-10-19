use std::time::Duration;
use tokio::time::sleep;

/// Waits 300 ms (pretend: a network call), then doubles `n`. Given.
pub async fn slow_double(n: u32) -> u32 {
    sleep(Duration::from_millis(300)).await;
    n * 2
}

/// "Hello, Alex!" for name = "Alex". Given.
pub async fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Two greetings one after the other, joined by a space: "Hello, Alex! Hello, Alex!"
pub async fn greet_twice(name: &str) -> String {
    // TODO: .await greet(name) twice (in sequence), then join the results
    todo!()
}

/// Returns (a doubled, b doubled). Both waits must overlap.
pub async fn double_both(a: u32, b: u32) -> (u32, u32) {
    // TODO: tokio::join! slow_double(a) and slow_double(b)
    todo!()
}

/// Doubles both and adds the results.
pub async fn sum_of_both(a: u32, b: u32) -> u32 {
    // TODO: join! the two calls, then add the results
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn double_both_returns_both_results() {
        let out = tokio::time::timeout(Duration::from_secs(5), double_both(3, 8))
            .await
            .expect("timed out");
        assert_eq!(out, (6, 16));
    }

    #[tokio::test]
    async fn double_both_runs_concurrently() {
        let start = Instant::now();
        let out = tokio::time::timeout(Duration::from_secs(5), double_both(1, 2))
            .await
            .expect("timed out");
        let took = start.elapsed();
        assert_eq!(out, (2, 4));
        assert!(
            took < Duration::from_millis(520),
            "took {took:?}: the two waits ran one after another, use join!"
        );
    }

    #[tokio::test]
    async fn greet_twice_awaits_both() {
        assert_eq!(greet_twice("Sam").await, "Hello, Sam! Hello, Sam!");
    }

    #[tokio::test]
    async fn sum_of_both_adds_results() {
        let start = Instant::now();
        let out = tokio::time::timeout(Duration::from_secs(5), sum_of_both(10, 20))
            .await
            .expect("timed out");
        assert_eq!(out, 60);
        assert!(start.elapsed() < Duration::from_millis(520), "use join! so the waits overlap");
    }
}
