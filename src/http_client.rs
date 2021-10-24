pub async fn http_request() {
    let body = reqwest::get("https://www.rust-lang.org")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("body = {:?}", body);
}
