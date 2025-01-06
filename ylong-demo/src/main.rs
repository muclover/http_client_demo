use std::error::Error;

use ylong_http_client::async_impl::{Body, ClientBuilder, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ClientBuilder::new().build().unwrap();
    let request = Request::builder()
        .method("GET")
        .url("http://httpbin.org/ip")
        .body(Body::empty())
        .unwrap();

    let mut resp = client.request(request).await?;
    println!("{}", resp.status());
    println!("{}", resp.headers());
    let mut buf = [0u8; 1024];
    let mut sum = 0;
    loop {
        let size = resp.data(&mut buf).await?;
        if size == 0 {
            break;
        }
        sum += size;
    }
    println!("{:?}", sum);
    println!("{:?}", &buf[..sum]);
    // 尝试将字节流转换为 UTF-8 字符串
    match String::from_utf8(buf[..sum].to_vec()) {
        Ok(valid_str) => println!("有效 UTF-8 字符串: {}", valid_str),
        Err(e) => println!("无效 UTF-8 编码: {:?}", e),
    }
    Ok(())
}
