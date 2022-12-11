fn main() {}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn get() {
        let body = reqwest::get("https://www.rust-lang.org")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("body = {:?}", body);
    }
}
