// equals fn foo() -> impl Future<Output = ()> { async { println!("hi") } }
async fn foo() {
    println!("hi")
}

#[cfg(test)]
mod tests {
    use super::foo;

    #[tokio::test] // use tokio macro block on call.
    async fn call() {
        /*
         * #[tokio::test] do this:
         * let runtime = tokio::runtime::Runtime::new().unwrap();
         * runtime.block_on(async {
         *     foo().await;
         * });
         */
        foo().await;
    }
}
