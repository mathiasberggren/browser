pub async fn get_webpage(url: &str) -> Result<String, reqwest::Error> {
    println!("Fetching url: {}", url);
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    println!("Response: {:?} {}", res.version(), res.status());

    let body = res.text().await?;
    Ok(body)
}
