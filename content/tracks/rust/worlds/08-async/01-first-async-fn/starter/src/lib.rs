/// "Hello, Alex!" for name = "Alex".
pub async fn greet(name: &str) -> String {
    // TODO: build the greeting with format!
    todo!()
}

pub async fn add_async(a: i32, b: i32) -> i32 {
    // TODO: return the sum
    todo!()
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
}
