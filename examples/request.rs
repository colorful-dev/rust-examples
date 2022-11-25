use anyhow::Result;
use http::HeaderMap;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.rust-lang.org";
    let client = reqwest::Client::new();
    // let headers = HeaderMap::new();
    let resp = client
        .get(url)
        // .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", resp);
    Ok(())
}
