#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::Client::builder()
        .gzip(true)
        .build()
        .unwrap()
        .get("http://httpbin.org/gzip")
        .send()
        .await?;
    println!("{:#?}", response);
    let header = response.headers();
    println!("{:#?}", header);
    let bytes = response.bytes().await?;
    println!("bytes: {:?}", bytes);
    Ok(())
}
