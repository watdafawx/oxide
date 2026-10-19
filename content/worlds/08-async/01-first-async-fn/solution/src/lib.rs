/// "Hello, Alex!" for name = "Alex".
pub async fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

/// Two greetings, joined by a space: "Hello, Alex! Hello, Alex!"
pub async fn greet_twice(name: &str) -> String {
    let first = greet(name).await;
    let second = greet(name).await;
    format!("{first} {second}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn greet_returns_hello_name() {
        assert_eq!(greet("Alex").await, "Hello, Alex!");
    }

    #[tokio::test]
    async fn add_async_sums() {
        assert_eq!(add_async(2, 40).await, 42);
        assert_eq!(add_async(-5, 5).await, 0);
    }

    #[tokio::test]
    async fn greet_twice_awaits_both() {
        assert_eq!(greet_twice("Sam").await, "Hello, Sam! Hello, Sam!");
    }
}
