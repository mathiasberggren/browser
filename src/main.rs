use html5ever::{parse_document};
// use http::{Request};
// use std::net::Shutdown::Write;

use reqwest::get;

#[tokio::main]
async fn main() {
    let body = get_webpage("https://www.rust-lang.org").await.unwrap();
    parse_document(body, TokenizerOpts(true, true, true));

    println!("This is the body {:?}", body);

}

async fn get_webpage(url: &str) -> Option<String> {
    let body = get(url)
    .await
    .ok()?
    .text()
    .await
    .ok()?;

    return Some(body);
}
